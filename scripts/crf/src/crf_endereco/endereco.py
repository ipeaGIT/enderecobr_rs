import re
import random
from dataclasses import dataclass, asdict, field
from typing import Literal


@dataclass
class Endereco:
    logradouro: str
    numero: str
    complemento: str
    bairro: str
    municipio: str
    uf: str
    cep: str
    endereco_formatado: str | None = None


@dataclass
class EnderecoParametros:
    abreviar_campos: set[str] = field(default_factory=set)
    excluir_palavra_campos: set[str] = field(default_factory=set)
    misturar_municipio_uf: bool = False
    separador_uf: str = " "
    padrao_numero_inexistente: str = "S/N"
    numero_separado_milhar: bool = False
    prefixo_numero: str | None = None
    formato: str = "$logradouro $numero $complemento $bairro $municipio"
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
                ["S/N", "SN", "S N", "SEM NUMERO", "SEM NUM"]
            ),
            numero_separado_milhar=self.rnd.random() < 0.05,
            prefixo_numero=None
            if self.rnd.random() > 0.95
            else self.rnd.choice(["N.", "NUM", "NO", "Nº"]),
            formato=self.rnd.choice(
                [
                    # Ordem usual
                    "$logradouro $numero $complemento",
                    "$logradouro $numero $complemento $bairro",
                    "$logradouro $numero $complemento $bairro $municipio",
                    "$logradouro $numero $complemento $bairro $municipio $cep",
                    # cep intercalado
                    "$logradouro $numero $complemento $cep $bairro $municipio",
                    "$logradouro $numero $complemento $bairro $cep $municipio",
                    # ordem inversa
                    "$municipio $bairro $logradouro $numero $complemento",
                    # Bairro entre complemento e número
                    "$logradouro $bairro $numero $complemento $municipio",
                    "$logradouro $numero $bairro $complemento",
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
    def __init__(self, abreviacoes: dict[str, list[str]] = {}):
        self.abreviacoes = abreviacoes

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
            # if params.erro_digitacao_campos.get(campo):
            #     reg[campo] = self._erro_digitacao(reg[campo])
            # if params.excluir_palavra_campos.get(campo):
            #     reg[campo] = self._excluir_palavra(reg[campo])

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
        if params.formato_cep == "99999-999":
            valor = f"{valor[:5]}-{valor[5:]}"
        elif params.formato_cep == "99.999-999":
            valor = f"{valor[:2]}.{valor[2:5]}-{valor[5:]}"
        reg["cep"] = valor

        reg["endereco_formatado"] = self._aplicar_formato(
            params.formato, reg, params.separador
        )
        return Endereco(**reg)

    # ---------------------------------------------------

    def _aplicar_formato(
        self, formato: str, reg: dict[str, str], separador: str
    ) -> str:
        resultado = re.sub(r"\s+", "$separador", formato)
        for campo in [
            "logradouro",
            "numero",
            "complemento",
            "bairro",
            "municipio",
            "cep",
        ]:
            resultado = resultado.replace(f"${campo}", reg.get(campo, ""))
        resultado = re.sub(r"\s+", " ", resultado.strip())
        return re.sub(r"(\$separador)+", separador, resultado)

    def _abreviar_texto(self, texto: str) -> str:
        for termo, formas in self.abreviacoes.items():
            padrao = re.compile(rf"\b{termo}\b", flags=re.IGNORECASE)
            if padrao.search(texto):
                texto = padrao.sub(formas[0], texto)  # determinístico
        return texto

    def _erro_digitacao(self, texto: str) -> str:
        if len(texto) > 2:
            return texto[:-1] + texto[-1].swapcase()  # exemplo fixo
        return texto

    def _excluir_palavra(self, texto: str) -> str:
        palavras = texto.split()
        if len(palavras) > 2:
            del palavras[1]
        return " ".join(palavras)
