// Cada processo terá um nome e o PID(que nesse exemplo o usuário que definirá)
#[derive(Debug)]
pub struct Processo {
    pub pid: u32,
    pub nome: String,
}

// Criação da pilha como um vetor
pub struct PilhaDeProcessamento {
    pub pilha: Vec<Processo>,
}

impl PilhaDeProcessamento {
    // Cria a Pilha
    pub fn new() -> PilhaDeProcessamento {
        PilhaDeProcessamento { pilha: Vec::new() }
    }
    // Adiciona processo
    pub fn adiciona(&mut self, processo: Processo) {
        self.pilha.push(processo);
    }
    // Mostra o topo da pilha -> primeiro elemento
    pub fn topo_pilha(&self) -> Option<&Processo> {
        self.pilha.last()
    }

    // Remove processo -> topo da pilha
    pub fn remove(&mut self) -> Option<Processo> {
        self.pilha.pop()
    }

    // Lista todos os processos em ordem de pilha
    pub fn mostra_todos_elementos(&self) {
        println!("\nPilha Completa:");
        for processos in self.pilha.iter().rev() {
            println!("{:?}", processos.nome);
        }
    }
}






