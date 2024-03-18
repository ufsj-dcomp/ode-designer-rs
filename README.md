# ODE Designer

## Introdução

Abaixo estão a descrição do software, como instalá-lo e uma breve introdução de como usá-lo.

## Sobre o Software

Este software foi desenvolvido na Universidade Federal de São João Del-Rei ([UFSJ](https://ufsj.edu.br)) em parceria com a Fundação de Amparo à Pesquisa do Estado de Minas Gerais ([FAPEMIG](http://www.fapemig.br/pt/)).

O software tem como o objetivo facilitar o desenvolvimento de Equações Diferenciais Ordinárias (EDOs), tendo como o alvo não somente o auxílio de pesquisadores na área, mas também o ensino-aprendizagem de modelagem computacional.

Para isso, o software provê uma interface gráfica com um editor baseados em nós, no qual o usuário pode desenhar o sistema de EDOs por meio dos componentes representados pelos nós.

## Funcionalidades

### Editor baseado em nós

![Imagem do editor de nós no software, contendo os nós 'Var', 'Const', 'grow' e 'dVar/dt', que constroem a EDO dVar/dt = Var*Const](readme/demo-nodes.png)

### Plotagem diretamente no software

![Plotagem da EDO dVar/dt = Var*Const nos tempos 41 até 50, para os valores iniciais Var = 1 e Const = 2](readme/demo-simulation.png)

### Eportação do Código da Simulação em Python e PDF

```py
# imports of scipy and numpy omitted

def initial_values() -> np.ndarray:
    Var_0 = 1.0
    return np.array((Var_0,))


def constants() -> list:
    Const = 2.0
    return [Const]


def variable_names() -> list[str]:
    return ["Var"]


def system(t: np.float64, y: np.ndarray, *constants) -> np.ndarray:
    Var, = y
    Const, = constants
    
    dVar_dt = Var*Const 

    return np.array([dVar_dt])

# Rest of the code used to simulate and plot to PDF omitted
```

### Estensibilidade via código em Python

Dado o seguinte código de Python:

```py
import math

@node
def sine(x):
    return math.sin(x)


@node(format="$1 ^ $2")
def power(x, y):
    return x ** y
```

Ao importá-lo no menu de gerenciamento extensões (em inglês, *Manage Extensions*), pode-se usar os nós definidos como se fossem nativos, como na imagem abaixo.

![O editor de nós incluindo os nós customizados de seno e potência](readme/demo-with-extensions-nodes.png)

O código pode ser usado para simular assim como nós nativos.

![Plotagem das EDOs utilizando nós customizados](readme/demo-with-extensions-simulation.png)
