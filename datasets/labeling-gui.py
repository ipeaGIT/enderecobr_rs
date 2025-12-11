#!/usr/bin/env -S uv run
# /// script
# requires-python = ">=3.10"
# dependencies = [
#   "gradio",
#   "duckdb",
# ]
# ///

import gradio as gr
import duckdb
import json
from datetime import datetime
from pathlib import Path

# Configurações
DATASET_PATH = "dados/dataset.parquet"
DB_PATH = "dados/annotations.duckdb"
DEFAULT_QUERY = f"SELECT * FROM '{DATASET_PATH}' ORDER BY random()"


# Estado global
class AnnotationState:
    def __init__(self):
        self.con = None
        self.current_example = None
        self.current_index = 0
        self.examples = []
        self.labels = []
        self.history = []
        self.query = DEFAULT_QUERY
        self.skip_annotated: bool = True

    def init_db(self):
        """Inicializa banco de dados e cria tabela se necessário"""
        Path(DB_PATH).parent.mkdir(parents=True, exist_ok=True)
        self.con = duckdb.connect(DB_PATH)
        self.con.execute("""
            CREATE TABLE IF NOT EXISTS annotations (
                example_data TEXT NOT NULL,
                label TEXT NOT NULL,
                notes TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        """)
        self.con.commit()

    def load_labels(self):
        """Carrega labels distintas do banco"""
        if self.con is None:
            self.init_db()
        result = self.con.execute(
            "SELECT DISTINCT label FROM annotations ORDER BY label"
        ).fetchall()
        self.labels = [row[0] for row in result]
        return self.labels

    def load_examples(self, query=None):
        """Carrega exemplos do dataset"""
        if query:
            self.query = query
        if self.con is None:
            self.init_db()

        try:
            result = self.con.execute(self.query).fetchall()
            columns = [desc[0] for desc in self.con.description]
            self.examples = [dict(zip(columns, row)) for row in result]
            self.current_index = 0
            return True, f"✓ {len(self.examples)} exemplos carregados"
        except Exception as e:
            return False, f"✗ Erro ao carregar: {str(e)}"

    def get_next_unannotated(self):
        """Obtém próximo exemplo não anotado"""
        if not self.examples:
            return None, "Nenhum exemplo carregado"

        # Buscar exemplos já anotados
        annotated = set()
        if self.skip_annotated:
            result = self.con.execute("SELECT example_data FROM annotations").fetchall()
            for row in result:
                annotated.add(row[0])

        # Procurar próximo não anotado
        start_idx = self.current_index
        while self.current_index < len(self.examples):
            example = self.examples[self.current_index]
            example_json = json.dumps(example, sort_keys=True, ensure_ascii=False)

            if example_json not in annotated:
                self.current_example = example
                return example, f"Exemplo {self.current_index + 1}/{len(self.examples)}"

            self.current_index += 1

        # Se chegou ao fim, voltar ao início
        self.current_index = 0
        if start_idx > 0:
            return self.get_next_unannotated()

        return None, "Todos os exemplos foram anotados"

    def save_annotation(self, label, notes):
        """Salva anotação no banco"""
        if self.current_example is None:
            return False, "Nenhum exemplo carregado"

        if not label:
            return False, "Selecione uma label"

        example_json = json.dumps(
            self.current_example, sort_keys=True, ensure_ascii=False
        )

        try:
            self.con.execute(
                """
                INSERT INTO annotations (example_data, label, notes, created_at)
                VALUES (?, ?, ?, ?)
            """,
                [example_json, label, notes or "", datetime.now()],
            )
            self.con.commit()

            # Adicionar ao histórico
            self.history.append(
                {
                    "example": self.current_example,
                    "label": label,
                    "notes": notes,
                    "action": "saved",
                    "index": self.current_index,
                }
            )
            if len(self.history) > 5:
                self.history.pop(0)

            self.current_index += 1
            return True, "✓ Anotação salva"
        except Exception as e:
            return False, f"✗ Erro ao salvar: {str(e)}"

    def skip_example(self):
        """Pula exemplo atual"""
        if self.current_example is None:
            return False, "Nenhum exemplo carregado"

        self.history.append(
            {
                "example": self.current_example,
                "action": "skipped",
                "index": self.current_index,
            }
        )
        if len(self.history) > 5:
            self.history.pop(0)

        self.current_index += 1
        return True, "Exemplo pulado"

    def go_back(self):
        """Volta para o último exemplo do histórico"""
        if not self.history:
            return False, "Histórico vazio"

        last = self.history.pop()
        self.current_example = last["example"]
        self.current_index = last["index"]

        return True, f"Voltou para exemplo {self.current_index + 1}"

    def add_label(self, new_label):
        """Adiciona nova label"""
        if not new_label or not new_label.strip():
            return False, "Label vazia", self.labels

        new_label = new_label.strip()
        if new_label in self.labels:
            return False, "Label já existe", self.labels

        self.labels.append(new_label)
        self.labels.sort()

        return True, f"✓ Label '{new_label}' adicionada", self.labels


state = AnnotationState()


# Funções da interface
def init_interface():
    """Inicializa interface carregando dados"""
    state.init_db()
    labels = state.load_labels()
    success, msg = state.load_examples()

    if success:
        example, status = state.get_next_unannotated()
        if example:
            return (
                json.dumps(example, indent=2, ensure_ascii=False),
                gr.update(choices=labels, value=None),
                "",
                status,
                msg,
            )

    return "", gr.update(choices=labels, value=None), "", "Carregue o dataset", msg


def save_and_next(label, notes):
    """Salva anotação e carrega próximo exemplo"""
    success, msg = state.save_annotation(label, notes)

    if success:
        example, status = state.get_next_unannotated()
        if example:
            return (
                json.dumps(example, indent=2, ensure_ascii=False),
                None,
                "",
                status,
                msg,
            )
        else:
            return "", None, "", status, msg

    return (
        json.dumps(state.current_example, indent=2, ensure_ascii=False)
        if state.current_example
        else "",
        label,
        notes,
        f"Exemplo {state.current_index + 1}/{len(state.examples)}"
        if state.examples
        else "",
        msg,
    )


def skip_and_next():
    """Pula exemplo e carrega próximo"""
    state.skip_example()
    example, status = state.get_next_unannotated()

    if example:
        return (
            json.dumps(example, indent=2, ensure_ascii=False),
            None,
            "",
            status,
            "Exemplo pulado",
        )

    return "", None, "", status, "Nenhum exemplo disponível"


def go_back_example():
    """Volta para exemplo anterior do histórico"""
    success, msg = state.go_back()

    if success and state.current_example:
        return (
            json.dumps(state.current_example, indent=2, ensure_ascii=False),
            None,
            "",
            f"Exemplo {state.current_index + 1}/{len(state.examples)}",
            msg,
        )

    return "", None, "", "", msg


def add_new_label(new_label):
    """Adiciona nova label e atualiza interface"""
    success, msg, labels = state.add_label(new_label)

    if success:
        return gr.update(choices=labels, value=None), "", msg

    return gr.update(choices=labels), new_label, msg


def reload_dataset(query, skip_annotated):
    """Recarrega dataset com nova query"""
    state.skip_annotated = skip_annotated
    success, msg = state.load_examples(query)

    if success:
        example, status = state.get_next_unannotated()
        if example:
            return (
                json.dumps(example, indent=2, ensure_ascii=False),
                None,
                "",
                status,
                msg,
            )

    return "", None, "", "Erro ao carregar", msg


# Interface Gradio
with gr.Blocks(title="Anotação de Datasets") as demo:
    gr.Markdown("# 🏷️ Interface de Anotação de Datasets")

    with gr.Row():
        with gr.Column(scale=4):
            status_text = gr.Textbox(
                label="Status", interactive=False, value="Inicializando..."
            )
        with gr.Column(scale=1):
            msg_text = gr.Textbox(label="Mensagem", interactive=False)

    # Exemplo atual
    gr.Markdown("### 📋 Exemplo Atual")
    example_json = gr.Code(
        label="Dados do Exemplo", language="json", lines=10, interactive=False
    )

    # Anotação
    gr.Markdown("### ✏️ Anotação")
    with gr.Row():
        label_radio = gr.Radio(label="Selecione a Label", choices=[], interactive=True)

    notes_text = gr.TextArea(
        label="Notas (opcional)",
        placeholder="Adicione observações sobre este exemplo...",
        lines=3,
    )

    with gr.Row():
        save_btn = gr.Button("💾 Salvar (Ctrl+Enter)", variant="primary", scale=2)
        skip_btn = gr.Button("⏭️ Pular", scale=1)
        back_btn = gr.Button("↩️ Voltar", scale=1)

    # Adicionar label
    gr.Markdown("### ➕ Adicionar Nova Label")
    with gr.Row():
        new_label_text = gr.Textbox(
            label="Nova Label", placeholder="Digite o nome da nova label...", scale=3
        )
        add_label_btn = gr.Button("Adicionar", scale=1)

    # Query do dataset
    with gr.Accordion("⚙️ Configuração da Query", open=False):
        query_text = gr.Code(
            label="Query DuckDB", language="sql", value=DEFAULT_QUERY, lines=3
        )
        skip_annotated = gr.Checkbox(label="Pular já anotados", value=True)
        reload_btn = gr.Button("🔄 Recarregar Dataset")

    # Eventos
    demo.load(
        init_interface,
        outputs=[example_json, label_radio, notes_text, status_text, msg_text],
    )

    save_btn.click(
        save_and_next,
        inputs=[label_radio, notes_text],
        outputs=[example_json, label_radio, notes_text, status_text, msg_text],
    )

    # Atalho Ctrl+Enter
    notes_text.submit(
        save_and_next,
        inputs=[label_radio, notes_text],
        outputs=[example_json, label_radio, notes_text, status_text, msg_text],
    )

    skip_btn.click(
        skip_and_next,
        outputs=[example_json, label_radio, notes_text, status_text, msg_text],
    )

    back_btn.click(
        go_back_example,
        outputs=[example_json, label_radio, notes_text, status_text, msg_text],
    )

    add_label_btn.click(
        add_new_label,
        inputs=[new_label_text],
        outputs=[label_radio, new_label_text, msg_text],
    )

    reload_btn.click(
        reload_dataset,
        inputs=[query_text, skip_annotated],
        outputs=[example_json, label_radio, notes_text, status_text, msg_text],
    )

if __name__ == "__main__":
    demo.launch()
