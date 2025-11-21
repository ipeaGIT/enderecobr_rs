from collections.abc import Iterable

def padronizar_logradouros(valor: str) -> str:
    """
    Logradouros

    >>> padronizar_logradouros("R DAS ROSAS")
    R DAS ROSAS
    """
    pass

def padronizar_numeros(valor: str) -> str: ...
def padronizar_complementos(valor: str) -> str: ...
def padronizar_bairros(valor: str) -> str: ...
def padronizar_municipios(valor: str) -> str: ...
def padronizar_estados_para_nome(valor: str) -> str: ...
def padronizar_tipo_logradouro(valor: str) -> str: ...
def padronizar_cep_leniente(valor: str) -> str: ...

### Padronizadores
def obter_padronizador_logradouros() -> Padronizador:
    pass

def obter_padronizador_numeros() -> Padronizador: ...

class Padronizador:
    """
    TODO
    """
    def adicionar_substituicoes(self, pares: Iterable[Iterable[str | None]]) -> None:
        """
        TODO
        """
        pass
    def padronizar(self, valor: str) -> str:
        """
        TODO
        """
        pass
    def obter_substituicoes(self) -> list[tuple[str | None]]:
        """
        TODO
        """
        pass
