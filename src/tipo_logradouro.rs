use std::sync::LazyLock;

use crate::Padronizador;

pub fn criar_padronizador_tipo_logradouro() -> Padronizador {
    let mut padronizador = Padronizador::default();
    padronizador
        // Substituição nova
        .adicionar(r"\s{2,}", " ")

        .adicionar(r"\.\.+", ".")         // remover pontos repetidos
        .adicionar(r"(\d+)\.(\d{3})", "$1$2") // pontos usados como separador de milhares

        .adicionar(r"\.([^ ])", ". $1") // garantir que haja espaco depois do ponto
        .adicionar(r" (-|\.) ", " ")
        .adicionar(r"\.$", "") // remocao de ponto final

        .adicionar(r"\.([^ ])", ". $1") // garantir que haja espaco depois do ponto

        // sinalizacao
        .adicionar("\"", "'") // existem ocorrencias em que aspas duplas sao usadas para se referir a um logradouro/quadra com nome relativamente ambiguo - e.g. RUA \"A\", 26. isso pode causar um problema quando lido com o data.table: https://github.com/Rdatatable/data.table/issues/4779. por enquanto, substituindo por aspas simples. depois a gente pode ver o que fazer com as aspas simples rs.

        // valores non-sense
        .adicionar(r"^-+$", "") // - --+ 0 00+
        // PS: A regex original era ^([^\d])\1{1,}$ que usa uma back-reference.
        // Ou seja, qualquer coisa que comece com algo que não seja um com um dígito e repete ele até o fim da string, pelo menos uma vez.
        // O motor do Rust não permite esse tipo de coisa. Troquei para os casos concretos.
        // FIXME: Precisa colocar pontuação também aqui ou retirar casos não permitidos.
        .adicionar(r"^(AA+|BB+|CC+|DD+|EE+|FF+|GG+|HH+|JJ+|KK+|LL+|MM+|NN+|OO+|PP+|QQ+|RR+|SS+|TT+|UU+|VV+|WW+|YY+|ZZ+|[*][*]+|__+|;;+|//+|,,+|::+|''+)$", "") // qualquer valor não numérico ou romano repetido 2+ vezes

        .adicionar(r"^\d+$", "") // tipos de logradouro não podem ser números

        // ordenacao de logradouros - e.g. 3A RUA, 15A TRAVESSA, 1A RODOVIA, 1O BECO, etc
        .adicionar(r"\b\d+(A|O) ?", "")

        // tipos de logradouro
        // problema visto no cadunico 2011: muitos tipos são truncados em 3 letras.
        // existem ambiguidades com CAM (CAMINHO x CAMPO), CON (CONJUNTO x
        // CONDOMINIO), PAS (PASSARELA x PASSAGEM x PASSEIO), entre outros. nesses
        // casos, acho melhor não "tomar um lado" e manter inalterado

        .adicionar(r"\bR(A|U)?\b\.?", "RUA")
        .adicionar(r"\b(ROD|RDV)\b\.?", "RODOVIA")
        .adicionar(r"\bAV(E|N|D|DA|I)?\b\.?", "AVENIDA")
        .adicionar(r"\bESTR?\b\.?", "ESTRADA") // EST pode ser ESTANCIA, mas são poucos casos. no cadunico 2011 ESTRADA eram 139780 e ESTANCIA 158, 0.1%
        .adicionar(r"\b(PCA?|PR(A|C))\b\.?", "PRACA")
        // regexp original: \bBE?CO?\b(?<!BECO)\.?
        .adicionar_com_ignorar(r"\bBE?CO?\b\.?", "BECO", r"\bBE?CO?\bBECO\.?") // (?<!BECO) serve para remover os matches com a palavra BECO ja correta 
        .adicionar(r"\b(T(RA?)?V|TRA)\b\.?", "TRAVESSA")
        .adicionar(r"\bP((A?R)?Q|QU?E)\b\.?", "PARQUE")
        // Regexp original: (?<!RODOVIA )\bAL(A|M)?\b\.?
        .adicionar_com_ignorar(r"\bAL(A|M)?\b\.?", "ALAMEDA", r"RODOVIA \bAL(A|M)?\b\.?") // evitando um possivel caso de RODOVIA AL ..., que faria referencia a uma rodovia estadual de alagoas
        .adicionar(r"\bLOT\b\.?", "LOTEAMENTO")
        .adicionar(r"\bVI?L\b\.?", "VILA")
        .adicionar(r"\bLAD\b\.?", "LADEIRA")
        .adicionar(r"\bDIS(TR?)?\b\.?", "DISTRITO")
        .adicionar(r"\bNUC\b\.?", "NUCLEO")
        .adicionar(r"\bL(AR|RG|GO)\b\.?", "LARGO")
        .adicionar(r"\bAER(OP)?\b\.?", "AEROPORTO")
        .adicionar(r"\bFAZ(EN?)?\b\.?", "FAZENDA")
        .adicionar(r"\bCOND\b\.?", "CONDOMINIO")
        .adicionar(r"\bSIT\b\.?", "SITIO")
        .adicionar(r"\bRES(ID)?\b\.?", "RESIDENCIAL")
        .adicionar(r"\bQ(U(AD?)?|D(RA?)?)\b\.?", "QUADRA")
        .adicionar(r"\bCHAC\b\.?", "CHACARA") // CHA pode ser CHAPADAO
        .adicionar(r"\bCPO\b\.?", "CAMPO")
        .adicionar(r"\bCOL\b\.?", "COLONIA")
        .adicionar(r"\bC(ONJ|J)\b\.?", "CONJUNTO")
        .adicionar(r"\bJ(D(I?M)?|A?RD|AR(DIN)?)\b\.?", "JARDIM")
        .adicionar(r"\bFAV\b\.?", "FAVELA")
        .adicionar(r"\bNUC\b\.?", "NUCLEO")
        .adicionar(r"\bVIE\b\.?", "VIELA")
        .adicionar(r"\bSET\b\.?", "SETOR")
        .adicionar(r"\bILH\b\.?", "ILHA")
        .adicionar(r"\bVER\b\.?", "VEREDA")
        .adicionar(r"\bACA\b\.?", "ACAMPAMENTO")
        .adicionar(r"\bACE\b\.?", "ACESSO")
        .adicionar(r"\bADR\b\.?", "ADRO")
        .adicionar(r"\bALT\b\.?", "ALTO")
        .adicionar(r"\bARE\b\.?", "AREA")
        .adicionar(r"\bART\b\.?", "ARTERIA")
        .adicionar(r"\bATA\b\.?", "ATALHO")
        .adicionar(r"\bBAI\b\.?", "BAIXA")
        .adicionar(r"\bBLO\b\.?", "BLOCO")
        .adicionar(r"\bBOS\b\.?", "BOSQUE")
        .adicionar(r"\bBOU\b\.?", "BOULEVARD")
        .adicionar(r"\bBUR\b\.?", "BURACO")
        .adicionar(r"\bCAI\b\.?", "CAIS")
        .adicionar(r"\bCAL\b\.?", "CALCADA")
        .adicionar(r"\bELE\b\.?", "ELEVADA")
        .adicionar(r"\bESP\b\.?", "ESPLANADA")
        .adicionar(r"\bFEI\b\.?", "FEIRA")
        .adicionar(r"\bFER\b\.?", "FERROVIA")
        .adicionar(r"\bFON\b\.?", "FONTE")
        .adicionar(r"\bFOR\b\.?", "FORTE")
        .adicionar(r"\bGAL\b\.?", "GALERIA")
        .adicionar(r"\bGRA\b\.?", "GRANJA")
        .adicionar(r"\bMOD\b\.?", "MODULO")
        .adicionar(r"\bMON\b\.?", "MONTE")
        .adicionar(r"\bMOR\b\.?", "MORRO")
        .adicionar(r"\bPAT\b\.?", "PATIO")
        .adicionar(r"\bPOR\b\.?", "PORTO")
        .adicionar(r"\bREC\b\.?", "RECANTO")
        .adicionar(r"\bRET\b\.?", "RETA")
        .adicionar(r"\bROT\b\.?", "ROTULA")
        .adicionar(r"\bSER\b\.?", "SERVIDAO")
        .adicionar(r"\bSUB\b\.?", "SUBIDA")
        .adicionar(r"\bTER\b\.?", "TERMINAL")
        .adicionar(r"\bTRI\b\.?", "TRINCHEIRA")
        .adicionar(r"\bTUN\b\.?", "TUNEL")
        .adicionar(r"\bUNI\b\.?", "UNIDADE")
        .adicionar(r"\bVAL\b\.?", "VALA")
        .adicionar(r"\bVAR\b\.?", "VARIANTE")
        .adicionar(r"\bZIG\b\.?", "ZIGUE-ZAGUE")
        .adicionar("OUTROS", "");

    // EDF é usado pra sinalizar endereços típicos do DF no CadUnico (sigla de
    // Endereço do DF), não substituir por EDIFICIO
    //  * pelo menos é o que diz o manual do CadUnico, mas isso não aparece nenhuma vez, pelo visto

    padronizador.preparar();
    padronizador
}

// Em Rust, a constant é criada durante a compilação, então só posso chamar funções muito restritas
// quando uso `const`. Nesse caso,  como tenho uma construção complexa da struct `Padronizador`,
// tenho que usar static com inicialização Lazy (o LazyLock aqui previne condições de corrida).
static PADRONIZADOR: LazyLock<Padronizador> = LazyLock::new(criar_padronizador_tipo_logradouro);

/// Padroniza uma string representando complementos de logradouros.
///
/// # Exemplo
/// ```
/// use enderecobr_rs::padronizar_complemento;
/// assert_eq!(padronizar_complemento("QD1 LT2 CS3"), "QUADRA 1 LOTE 2 CASA 3");
/// assert_eq!(padronizar_complemento("APTO. 405"), "APARTAMENTO 405");
/// ```
///
/// # Detalhes
/// Operações realizadas durante a padronização:
/// - remoção de espaços em branco antes e depois das strings e remoção de espaços em excesso entre palavras;
/// - conversão de caracteres para caixa alta;
/// - remoção de acentos e caracteres não ASCII;
/// - adição de espaços após abreviações sinalizadas por pontos;
/// - expansão de abreviações frequentemente utilizadas através de diversas expressões regulares (regexes);
/// - correção de alguns pequenos erros ortográficos.
///
/// Note que existe uma etapa de compilação das expressões regulares utilizadas,
/// logo a primeira execução desta função pode demorar um pouco a mais.
///
pub fn padronizar_tipo_logradouro(valor: &str) -> String {
    // Forma de obter a variável lazy
    let padronizador = &*PADRONIZADOR;
    padronizador.padronizar(valor)
}
