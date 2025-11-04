import re
import random
from dataclasses import dataclass, asdict, field
from typing import Callable, Literal


@dataclass
class Endereco:
    logradouro: str
    numero: str
    complemento: str
    bairro: str
    municipio: str
    uf: str
    cep: str
    formato: str = "logradouro numero complemento bairro municipio"
    separador: str = ", "


@dataclass
class EnderecoParametros:
    abreviar_campos: set[str] = field(default_factory=set)
    excluir_palavra_campos: set[str] = field(default_factory=set)
    misturar_municipio_uf: bool = False
    separador_uf: str = " "
    padrao_numero_inexistente: str = "S/N"
    numero_separado_milhar: bool = False
    prefixo_numero: str | None = None
    formato: str = "logradouro numero complemento bairro municipio"
    separador: str = ", "
    formato_cep: Literal["99999-999"] | Literal["99.999-999"] | Literal["99999999"] = (
        "99999999"
    )
    erro_digitacao_campos: set[str] = field(default_factory=set)
    erro_digitacao_posicao: float = 0.5


class GeradorParametrosEndereco:
    def __init__(self, seed: int | None = None) -> None:
        self.rnd = random.Random(seed)

    def gerar_parametro(self) -> EnderecoParametros:
        return EnderecoParametros(
            abreviar_campos={
                c
                for c in ["logradouro", "bairro", "municipio", "complemento"]
                if self.rnd.random() < 0.2
            },
            excluir_palavra_campos={
                c
                for c in ["logradouro", "bairro", "municipio", "complemento"]
                if self.rnd.random() < 0.1
            },
            misturar_municipio_uf=self.rnd.random() < 0.1,
            separador_uf=self.rnd.choice([" ", "-", " - ", "/"]),
            padrao_numero_inexistente=self.rnd.choice(
                ["", "S/N", "SN", "S N", "SEM NUMERO", "SEM NUM"]
            ),
            numero_separado_milhar=self.rnd.random() < 0.05,
            prefixo_numero=None
            if self.rnd.random() > 0.95
            else self.rnd.choice(["N.", "NUM", "NO", "Nº"]),
            formato=self.rnd.choice(
                [
                    # Ordem usual
                    "logradouro numero complemento",
                    "logradouro numero complemento bairro",
                    "logradouro numero complemento bairro municipio",
                    "logradouro numero complemento bairro municipio cep",
                    # cep intercalado
                    "logradouro numero complemento cep bairro municipio",
                    "logradouro numero complemento bairro cep municipio",
                    # ordem inversa
                    "municipio bairro logradouro numero complemento",
                    # Bairro entre complemento e número
                    "logradouro bairro numero complemento municipio",
                    "logradouro numero bairro complemento",
                ]
            ),
            separador=self.rnd.choice([", ", " "]),
            formato_cep=self.rnd.choice(["99999-999", "99.999-999", "99999999"]),
            erro_digitacao_campos={
                c
                for c in ["logradouro", "bairro", "municipio", "complemento"]
                if self.rnd.random() < 0.1
            },
            erro_digitacao_posicao=self.rnd.random(),
        )


class EnderecoGerador:
    def __init__(
        self,
        abreviacoes: dict[str, list[str]] | None = None,
        tokenizer: Callable[[str], list[str]] | None = None,
    ):
        self.abreviacoes: dict[str, list[str]] = abreviacoes or dict()
        self.tokenizer: Callable[[str], list[str]] | None = tokenizer

    def gerar_endereco(
        self, endereco: Endereco, params: EnderecoParametros
    ) -> Endereco:
        reg = {
            k: v.upper() if isinstance(v, str) else v
            for k, v in asdict(endereco).items()
        }

        for campo in ["logradouro", "bairro", "municipio", "complemento"]:
            if not reg.get(campo):
                continue
            if campo in params.abreviar_campos:
                reg[campo] = self._abreviar_texto(reg[campo])
            if campo in params.erro_digitacao_campos:
                reg[campo] = self._erro_digitacao(reg[campo])
            if campo in params.excluir_palavra_campos:
                reg[campo] = self._excluir_palavra(reg[campo])

        if params.misturar_municipio_uf:
            reg["municipio"] = f"{reg['municipio']}-{reg['uf']}"

        valor = reg.get("numero", "")
        if not valor:
            reg["numero"] = params.padrao_numero_inexistente
        elif valor.isdecimal():
            if params.numero_separado_milhar:
                valor = f"{int(valor):,}"
            if params.prefixo_numero:
                valor = f"{params.prefixo_numero} {valor}"
            reg["numero"] = valor

        valor = str(reg.get("cep", "")).zfill(8)
        match params.formato_cep:
            case "99.999-999":
                valor = f"{valor[:2]}.{valor[2:5]}-{valor[5:]}"
            case "99999-999":
                valor = f"{valor[:5]}-{valor[5:]}"
            case "99999999":
                pass
        reg["cep"] = valor

        reg["formato"] = params.formato
        reg["separador"] = params.separador
        return Endereco(**reg)

    # ---------------------------------------------------

    def _abreviar_texto(self, texto: str) -> str:
        for termo, formas in self.abreviacoes.items():
            padrao = re.compile(rf"\b{termo}\b", flags=re.IGNORECASE)
            if padrao.search(texto):
                texto = padrao.sub(formas[0], texto)  # determinístico
        return texto

    def _erro_digitacao(self, texto: str) -> str:
        # if len(texto) > 2:
        #     return texto[:-1] + texto[-1].swapcase()  # exemplo fixo
        # return texto
        # Nada, por enquanto
        return texto

    def _excluir_palavra(self, texto: str) -> str:
        palavras = texto.split()
        if len(palavras) > 2:
            del palavras[1]
        return " ".join(palavras)


ABREVIACOES_COMUNS = {
    "RUA": ["RU", "R", "R.", "R,", "RUA R", "RUA RU", "RUA R.", "RUA R,"],
    "RODOVIA": [
        "ROD",
        "RDV",
        "ROD.",
        "RDV.",
        "ROD,",
        "RDV,",
        "RUA ROD",
        "RUA RDV",
        "RODOVIA ROD",
        "RODOVIA RDV",
        "RODOVIA-",
    ],
    "AVENIDA": [
        "AV",
        "AVE",
        "AVN",
        "AVD",
        "AVDA",
        "AVI",
        "AV.",
        "AV,",
        "RUA AV",
        "RUA AVE",
        "RODOVIA AV",
        "AVENIDA AV",
        "AVENIDA-",
    ],
    "ESTRADA": [
        "EST",
        "ESTR",
        "ETR",
        "EST.",
        "ETR.",
        "RUA EST",
        "RODOVIA ESTR",
        "ESTRADA ESTR",
        "ESTRADA-",
    ],
    "PRACA": [
        "PCA",
        "PRC",
        "PCA.",
        "PRC.",
        "RUA PCA",
        "RUA PRC",
        "PRACA PCA",
        "PRACA PRC",
        "PRACA-",
    ],
    "BECO": ["BC", "BEC", "BE", "BE.", "BCO", "BECO", "RUA BECO", "BECO BE", "BECO-"],
    "TRAVESSA": [
        "TV",
        "TRV",
        "TRAV",
        "TRAV.",
        "RUA TRAV",
        "RODOVIA TRV",
        "TRAVESSA TRAV",
        "TRAVESSA-",
    ],
    "PARQUE": [
        "P",
        "PQ",
        "PQU",
        "PARQ",
        "PQUE",
        "PARQUE",
        "RUA PQ",
        "RODOVIA PARQUE",
        "PARQUE PQU",
        "PARQUE-",
    ],
    "ALAMEDA": [
        "ALA",
        "AL",
        "ALA.",
        "AL.",
        "RUA ALA",
        "ALAMEDA ALA",
        "ALAMEDA-",
        "RODOVIA ALA",
    ],
    "LOTEAMENTO": ["LOT", "LOT.", "RUA LOT", "LOTEAMENTO LOT", "LOTEAMENTO-"],
    "LOCALIDADE": ["LOC", "LOC.", "RUA LOC", "LOCALIDADE LOC", "LOCALIDADE-"],
    "VILA": ["VL", "VL.", "VILA VILA", "VILA-", "VILA ,"],
    "LADEIRA": ["LAD", "LAD.", "LADEIRA LAD", "LADEIRA-"],
    "DISTRITO": ["DT", "DISTR", "DISTR.", "DISTRITO DISTRITO", "DISTRITO-"],
    "NUCLEO": ["NUC", "NUC.", "NUCLEO NUCLEO", "NUCLEO-"],
    "LARGO": ["LRG", "LGO", "LARGO LRG", "LARGO LGO", "LARGO-"],
    "AEROPORTO": [
        "AER",
        "AERO",
        "AEROP",
        "AEROP.",
        "AEROPORTO AER",
        "AEROPORTO INTERNACIONAL",
    ],
    "CONDOMINIO": ["COND", "COND.", "RODOVIA COND", "CONDOMINIO COND"],
    "FAZENDA": ["FAZ", "FAZ.", "FAZEN", "FAZEN.", "FAZENDA FAZ", "FAZENDA-"],
    "COLONIA": ["COL", "COL.", "COL AGR", "COLONIA AGR", "COLONIA AGRICOLA"],
    "SANTA": ["STA", "STA.", "SA"],
    "SANTO": ["STO", "STO.", "S"],
    "NOSSA SENHORA": ["NSA", "NS", "NOSSA SRA", "NOSSA SENHORA", "N SRA"],
    "SENHOR DO BONFIM": ["SR BONFIM", "SENHOR BONFIM", "SR DO BONFIM"],
    "SENHOR": ["SR"],
    "NOSSO SENHOR": ["NS"],
    "ALMIRANTE": ["ALM", "ALTE", "ALTE."],
    "MARECHAL": ["MAL", "MAR."],
    "GENERAL": ["GEN", "GAL"],
    "SARGENTO": ["SGT", "SGTO", "SARG"],
    "PRIMEIRO-SARGENTO": ["1 SARGENTO", "PRIM SARGENTO"],
    "SEGUNDO-SARGENTO": ["2 SARGENTO", "SEG SARGENTO"],
    "TERCEIRO-SARGENTO": ["3 SARGENTO", "TERC SARGENTO"],
    "CORONEL": ["CEL"],
    "BRIGADEIRO": ["BRIG"],
    "TENENTE": ["TEN"],
    "TENENTE-CORONEL": ["TENENTE CORONEL"],
    "TENENTE-BRIGADEIRO": ["TENENTE BRIGADEIRO"],
    "TENENTE-AVIADOR": ["TENENTE AVIADOR"],
    "SUBTENENTE": ["SUB TENENTE"],
    "PRIMEIRO-TENENTE": ["1 TENENTE", "PRIM TENENTE"],
    "SEGUNDO-TENENTE": ["2 TENENTE", "SEG TENENTE"],
    "SOLDADO": ["SOLD"],
    "MAJOR": ["MAJ"],
    "PROFESSOR": ["PROF"],
    "PROFESSORA": ["PROFA"],
    "DOUTOR": ["DR"],
    "DOUTORA": ["DRA"],
    "ENGENHEIRO": ["ENG"],
    "ENGENHEIRA": ["ENGA"],
    "PADRE": ["PE."],
    "MONSENHOR": ["MONS"],
    "PRESIDENTE": ["PRES", "PRESID"],
    "GOVERNADOR": ["GOV"],
    "SENADOR": ["SEN"],
    "PREFEITO": ["PREF"],
    "DEPUTADO": ["DEP"],
    "VEREADOR": ["VER"],
    "ESPLANADA DOS MINISTERIOS": ["ESPL MIN", "ESPLANADA MINISTERIOS"],
    "MINISTRO": ["MIN", "MIN."],
    "JARDIM": ["JD", "JARD", "JAR DIM", "JAR.", "JARDIM"],
    "UNIDADE": ["UNID"],
    "CONJUNTO": ["CJ", "CONJ"],
    "LOTE": ["LT"],
    "LOTES": ["LTS"],
    "QUADRA": ["QDA"],
    "LOJA": ["LJ"],
    "LOJAS": ["LJS"],
    "APARTAMENTO": ["APTO", "APT"],
    "BLOCO": ["BL"],
    "SALAS": ["SLS"],
    "EDIFICIO": ["EDIF", "EDIF."],
    "EDIFICIO EMPRESARIAL": ["EDIF EMP", "ED EMP"],
    "CONDOMINIO (ABREV.)": ["COND"],
    "KM": ["KM."],
    "S/N": ["S N", "S.N", "S. N."],
    "ANDAR": ["1. ANDAR", "2 AND", "3. AND"],
    "ANDARES": ["2. ANDARES"],
    "CAIXA POSTAL": ["CX P", "C.P", "CX POSTAL", "CP POSTAL"],
    "DOM": ["D"],
    "INFANTE DOM": ["INF DOM", "INF D"],
    "GETULIO VARGAS": ["GETULHO VARGAS", "JETULHO VARGAS", "GET VARGAS", "JET VARGAS"],
    "JUSCELINO KUBITSCHEK": ["J. K.", "JUSC KUB", "JUSCELINO KUB", "JK"],
    "BEIRA MAR": ["BEIRA-MAR"],
    "RODOVIA BR-116": ["BR 116", "RODOVIA CENTO DEZESEIS", "RODOVIA CENTO E DEZESEIS"],
    "RODOVIA BR-101": ["BR 101", "RODOVIA CENTO E UM"],
}
