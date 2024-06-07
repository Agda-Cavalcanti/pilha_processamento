use crate::pilha::{Processo,PilhaDeProcessamento};
mod pilha;

fn main() {
    // Criação de processos
    let processo1 = Processo {
        pid: 1,
        nome: String::from("Processo 1"),
    };

    let processo2 = Processo {
        pid: 2,
        nome: String::from("Processo 2"),
    };

    let processo3 = Processo {
        pid: 3,
        nome: String::from("Processo 3"),
    };


    let processo4 = Processo {
        pid: 4,
        nome: String::from("Processo 4"),

    };

    let processo5 = Processo {
        pid: 5,
        nome: String::from("Processo 5"),

    };

    // Criando a pilha
    let mut pilha_processamento: PilhaDeProcessamento = PilhaDeProcessamento::new();

    // Adicionando os processos na pilha
    pilha_processamento.adiciona(processo1);
    pilha_processamento.adiciona(processo2);
    pilha_processamento.adiciona(processo3);
    pilha_processamento.adiciona(processo4);
    pilha_processamento.adiciona(processo5);

    // Mostrando a pilha completa
    pilha_processamento.mostra_todos_elementos();

    // Processo que está no topo da pilha(primeiro)
    if let Some(topo_pilha) = pilha_processamento.topo_pilha() {
        println!("\nProcesso a ser executado na pilha - PID: {:?}", topo_pilha.pid);
    }

    // Removendo processo
    if let Some(processo) = pilha_processamento.remove() {
        println!("\nProcesso removido: {:?}", processo.pid);
    }

    // Processo que está no topo da pilha(primeiro)
    if let Some(topo_pilha) = pilha_processamento.topo_pilha() {
        println!("\nProcesso a ser executado na pilha - PID: {:?}", topo_pilha.pid);
    }

    // Mostrando a pilha completa
   pilha_processamento.mostra_todos_elementos();

    // Processo que está no topo da pilha(primeiro)
    if let Some(topo_pilha) = pilha_processamento.topo_pilha() {
        println!("\nProcesso a ser executado na pilha - PID: {:?}", topo_pilha.pid);
    }
}



