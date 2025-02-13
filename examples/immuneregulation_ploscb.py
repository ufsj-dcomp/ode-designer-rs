import argparse, contextlib, sys, os
import scipy
import numpy as np
np.seterr(divide="raise")

# User extensions
def node(func=None, format=None):
    if func is None and format is not None:
        def inner(inners_func):
            return inners_func

        return inner

    else:
        return func

@node 
def inhibition(v, alpha, AC):
    return v/(1 + alpha*AC)

def initial_values() -> np.ndarray:
    AC_0 = 0.0
    CH_0 = 0.0
    Mreg_0 = 0.0
    N_0 = 0.0
    TD_0 = 10.0
    return np.array((
        AC_0,
        CH_0,
        Mreg_0,
        N_0,
        TD_0,
        ))


def constants() -> list:
    alpha = 0.5
    beta_AC = 0.5
    beta_N = 0.6
    beta_ch = 0.5
    gamma = 0.2
    k = 0.15
    m_AC = 0.3
    m_N = 0.3
    m_ch = 0.4
    m_mreg = 0.02
    s_N = 0.5
    return [
        alpha,
        beta_AC,
        beta_N,
        beta_ch,
        gamma,
        k,
        m_AC,
        m_N,
        m_ch,
        m_mreg,
        s_N,
        ]


def constants_with_names() -> list:
    constants_list = [
        ("alpha", 0.5),
        ("beta_AC", 0.5),
        ("beta_N", 0.6),
        ("beta_ch", 0.5),
        ("gamma", 0.2),
        ("k", 0.15),
        ("m_AC", 0.3),
        ("m_N", 0.3),
        ("m_ch", 0.4),
        ("m_mreg", 0.02),
        ("s_N", 0.5),
        ]
    return constants_list


def variable_names() -> list[str]:
    return [
        "AC",
        "CH",
        "Mreg",
        "N",
        "TD",
        ]


def system(t: np.float64, y: np.ndarray, *constants) -> np.ndarray:
    # populations
    AC,CH,Mreg,N,TD, = y
    # constants
    alpha,beta_AC,beta_N,beta_ch,gamma,k,m_AC,m_N,m_ch,m_mreg,s_N, = constants
    
    dAC_dt = (beta_AC*Mreg )+- (AC*m_AC ) 
    dCH_dt = (inhibition((beta_ch*N ),alpha,AC) )+- (m_ch*CH ) 
    dMreg_dt = (gamma*N )+- (m_mreg*Mreg ) 
    dN_dt = - (beta_N*N )+- (m_N*N )+(inhibition((s_N*(TD+CH ) ),alpha,AC) ) 
    dTD_dt = (beta_N*N )+- (TD*Mreg*k ) 

    return np.array([dAC_dt,dCH_dt,dMreg_dt,dN_dt,dTD_dt])

# includes! "ode-support.py"


def simulation_output_to_csv(sim_steps, simulation_output, write_to):
    if not simulation_output.success:
        print(simulation_output.message)
        return

    populatio_values_per_dt = simulation_output.y.T

    write_to.write(f"t,{','.join(variable_names())}\n")

    for dt, y in zip(sim_steps, populatio_values_per_dt):
        write_to.write(f"{dt},")
        write_to.write(",".join(f"{val:.4f}" for val in y))
        write_to.write("\n")


COLORS = [
    'tab:blue',
    'tab:orange',
    'tab:green',
    'tab:red',
    'tab:purple',
    'tab:brown',
    'tab:pink',
    'tab:gray',
    'tab:olive',
    'tab:cyan',
]

def plot_simulation(sim_steps, simulation_output, filename, x_label="time (days)", y_label="conc/ml"):
    import matplotlib.pyplot as plt
    from matplotlib.backends.backend_pdf import PdfPages

    with PdfPages(filename) as pdf:
        # All
        all_fig, all_ax = plt.subplots()
        all_fig.set_size_inches(8, 6)
        all_ax.set(title="", xlabel=x_label, ylabel=y_label)

        # Individually
        for i, (variable_name, variable_line_data) in enumerate(zip(variable_names(), simulation_output.y)):
            fig, ax = plt.subplots()
            fig.set_size_inches(8, 6)
            ax.set(
                title=variable_name,
                xlabel=x_label, 
                ylabel=y_label, 
            )            
            ax.plot(simulation_output.t, variable_line_data, color=COLORS[i % len(COLORS)])
            all_ax.plot(simulation_output.t, variable_line_data)

            pdf.savefig(fig)
        all_ax.legend(variable_names(),loc="best")
        pdf.savefig(all_fig)


def file_or_stdout(filename: str | None):
    if filename:
        return open(filename, 'w')
    else:
        return sys.stdout


def update_constants_with_params(constants, params):
    updated_constants = constants.copy()

    constant_names = [constant[0] for constant in constants]

    for name, value in params.items():
        for idx, (const_name, const_value) in enumerate(updated_constants):
            if const_name == name:
                updated_constants[idx] = (const_name, value)

    return updated_constants



def simulate(filename, st=0, tf=50, dt=0.1, plot=False, x_label="time (days)", y_label="conc/ml", params={}):
    sim_steps = np.arange(st, tf + dt, dt)

    constants_values = [value for _, value in update_constants_with_params(constants_with_names(), params)]

    simulation_output = scipy.integrate.solve_ivp(
        fun=system,
        t_span=(st, tf + dt * 2),
        y0=initial_values(),
        args=tuple(constants_values),
        t_eval=sim_steps,
    )

    if plot:
        plot_simulation(sim_steps, simulation_output, filename, x_label, y_label)
    else:
        with file_or_stdout(filename) as f:
            simulation_output_to_csv(sim_steps, simulation_output, f)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--st", type=float, default=0)
    parser.add_argument("--tf", type=float, default=50)
    parser.add_argument("--dt", type=float, default=0.01)
    parser.add_argument("-o", "--output", default=None)
    parser.add_argument("--csv", action=argparse.BooleanOptionalAction)
    parser.add_argument("--xlabel", type=str, default="time (days)")
    parser.add_argument("--ylabel", type=str, default="conc/ml")
    parser.add_argument("--params", type=str, default="")

    args = parser.parse_args()

    if args.params:
        params = {k: float(v) for k, v in (param.split('=') for param in args.params.split())}
    else:
        params = {}

    simulate(
        args.output,
        plot=not args.csv,
        st=args.st,
        tf=args.tf,
        dt=args.dt,
        x_label=args.xlabel,
        y_label=args.ylabel,
        params=params
    )