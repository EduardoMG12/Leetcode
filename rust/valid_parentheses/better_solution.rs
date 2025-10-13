fn validate_parentheses(s: String) -> bool {
    // Um Vec<char> servirá como nossa pilha.
    let mut stack: Vec<char> = Vec::new();

    // Vamos iterar por cada caractere da string.
    for c in s.chars() {
        // SE o caractere 'c' for um de ABERTURA ('(', '[', '{')
        if c == '(' || c == '[' || c == '{' {
            // O que você deve fazer com a pilha aqui?
            // Dica: use o método para adicionar à pilha.
            
            // ... seu código aqui ...

        } 
        // SENÃO, se o caractere 'c' for um de FECHAMENTO (')', ']', '}')
        else {
            // Primeiro, pegamos o último item da pilha.
            // .pop() retorna None se a pilha estiver vazia.
            let top = stack.pop();

            // Agora, precisamos verificar se o par é válido.
            // Por exemplo, se 'c' for ')', 'top' deveria ser '('.
            
            // O que acontece se 'top' for None (pilha vazia)? A string é válida?
            // E se o par não corresponder (ex: 'c' é ')' e 'top' é '[')?
            // O que a função deve retornar nesses casos de falha?

            // ... seu código aqui para validar o par ...
            // Dica: você pode usar uma expressão 'match' em 'top'.

        }
    }

    // No final, depois de passar por toda a string, como a pilha
    // deve estar para que a string seja considerada válida?
    // (Pense no caso como "(()" - o que sobraria na pilha?)
    
    // ... seu código aqui para o retorno final ...
}