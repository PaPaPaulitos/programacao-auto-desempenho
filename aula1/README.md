# Aula 01 - Threads


## Como funciona paralelismo?

Na programação normal o nosso programa executa todo o seu código em somente um núcleo do processador, sendo 100% mono thread, mas com o paralelismo podemos rodar outra parte de um programa em paralelo com a sua execução normal.

Exemplo desse código escrito em Rust, em que nós estamos rodando a aplicação e em paralelo vários cálculos de Fibonacci.

## No processador

A cada ciclo do processador ele roda uma fatia de tempo para cada Thread como no desenho abaixo.

![image](https://github.com/PaPaPaulitos/programacao-auto-desempenho/assets/87778216/c0bd9cf3-0d7e-47d9-bfc9-db7a4ae48fbf)

A Thread 0 é um programa de Fibonacci, assim como a 1,2 e 3, mas elas estão rodando em paralelo dentro do mesmo programa, sendo o seu tempo de execução sendo divido pelo processador.
