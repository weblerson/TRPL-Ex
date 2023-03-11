fn main() {
    // ***** VARIÁVEIS *****

    // Variáveis são imutáveis por natureza em Rust
    // Para torná-las mutáveis, adicionar a keyword "mut" antes do nome da variável

    let mut n = 5;
    println!("\nn: {}", n);

    n = 6;
    println!("n: {}", n);

    // CONSTANTES
    // São sempre imutáveis
    // Declaradas com "const" em vez de "let" e o tipo de valor deve ser anotado
    // Pode ser declaradas em qualquer escopo
    // Usar todas as letras em caixa alta com underline para separar as palavras

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("\nContante: {}", THREE_HOURS_IN_SECONDS);

    // SHADOWING
    // Declaração de uma nova variável com o mesmo nome de uma variável anterior
    // Não podemos alterar o tipo de uma variável

    let x = 5;
    let x = x + 1;

    {
        let x = x + 1;

        println!("\nO valor da variável no escopo interno é: {}", x);
    }

    println!("O valor da variável é: {}", x);

    // ***** TIPOS DE DADOS *****
    //
    // ## Tipos Escalares:
    // inteiros, flutuantes, booleanos e caracteres
    //
    // Inteiros: i{bits} (assinados) e u{bits} (não assinados)
    // bits: 8, 18, 32, 64 e 128. Também tem o arco: isize ou usize e dependem do computador
    //
    // Flutuantes: f{bits}
    // bits: 32 e 64.
    // Todos os flutuantes são assinados
    //
    // Booleanos: true ou false
    // Ocupam um byte
    //
    // Char: tipo de alfabeto mais primitivo
    // 4 bytes de tamanho
    // Aspas simples
    //
    // ## Tipos Compostos
    //
    // tuplas e arrays
    //
    // Tupla: vários valores com uma variedade de tipos
    // Imutáveis

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Para obter os valores individuais de uma tupla, podemos usar correspondência de padrões para
    // desestruturar um valor

    let (x, y, z) = tup;

    println!("\ny: {}", y);

    // Ou podemos resgatar por índice, usando um ponto

    let seis_ponto_quatro = tup.1;
    println!("Seis ponto quatro: {}", seis_ponto_quatro);

    // Matriz: agrupa elementos do mesmo tipo em um conjunto de comprimento fixo

    let a = [1, 2, 3, 4, 5];

    // Para tipar uma matriz, usa-se colchetes com o tipo de cada elemento, um ponto e vírgula e,
    // em seguida, o número de elementos na matriz:

    let a: [u32;5] = [1, 2, 3, 4, 5];

    // Também é possível inicializar uma amtriz para conter o mesmo valor para cada elemento
    // especificando o valor inicial, seguido por um ponto e vírgula e, em seguida, o comprimento
    // da matriz entre colchetes:

    let a = [3; 5]; // Saída: [3, 3, 3, 3, 3]

    // Os valores de uma matriz são acessados por indexação:

    let a: [u32; 5] = [1, 2, 3, 4, 5];

    let (first, second) = (a[0], a[1]);

    println!("\nFirst: {}; Second: {}", first, second);

    // ***** FUNÇÕES *****
    // Declaradas com a keyword "fn" seguido pelo nome da função
}
