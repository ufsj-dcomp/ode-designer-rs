import argparse, contextlib, sys, os
import scipy
import numpy as np

def initial_values() -> np.ndarray:
    A_0 = 10.0
    B_0 = 20.0
    return np.array((
        A_0,
        B_0,
        ))


def constants() -> list:
    k = 0.5
    return [
        k,
        ]


def variable_names() -> list[str]:
    return [
        "A",
        "B",
        ]


def system(t: np.float64, y: np.ndarray, *constants) -> np.ndarray:
    # populations
    A,B, = y

    # constants
    k, = constants
    
    dA_dt = A + B 
    dB_dt = (A + B ) * k 

    return np.array([dA_dt,dB_dt])


def simulation_output_to_csv(sim_steps, simulation_output, write_to) -> str:
    if not simulation_output.success:
        print(simulation_output.message)
        return

    populatio_values_per_dt = simulation_output.y.T

    write_to.write(f"t,{','.join(variable_names())}\n")

    for dt, y in zip(sim_steps, populatio_values_per_dt):
        write_to.write(f"{dt},")
        write_to.write(",".join(f"{val:.4f}" for val in y))
        write_to.write("\n")


def plot_simulation(sim_steps, simulation_output, filename):
    import matplotlib.pyplot as plt
    from matplotlib.backends.backend_pdf import PdfPages

    with PdfPages(filename) as pdf:
        # All
        all_fig, all_ax = plt.subplots()
        all_ax.grid()
        all_ax.set(title=__name__, xlabel="Time", ylabel="Concentration")

        # Individually
        for variable_name, variable_line_data in zip(variable_names(), simulation_output.y):
            fig, ax = plt.subplots()
            ax.grid()

            ax.set(
                title=variable_name,
                xlabel="Time",
                ylabel="Concentration",
            )

            ax.plot(simulation_output.t, variable_line_data)
            all_ax.plot(simulation_output.t, variable_line_data)

            pdf.savefig(fig)

        pdf.savefig(all_fig)


def file_or_stdout(filename: str | None):
    if filename:
        return open(filename, 'w')
    else:
        return sys.stdout


def simulate(filename, st=0, tf=10, dt=0.1, plot=False):
    sim_steps = np.arange(st, tf + dt, dt)

    simulation_output = scipy.integrate.solve_ivp(
        fun=system,
        t_span=(0, tf + dt),
        y0=initial_values(),
        args=constants(),
        t_eval=sim_steps,
    )

    if plot:
        plot_simulation(sim_steps, simulation_output, filename)

    else:
        with file_or_stdout(filename) as f:
            simulation_output_to_csv(sim_steps, simulation_output, f)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()

    parser.add_argument("--st", type=float, default=0)
    parser.add_argument("--tf", type=float, default=10)
    parser.add_argument("--dt", type=float, default=0.1)
    parser.add_argument("-o", "--output", default=None)
    parser.add_argument("--csv", action=argparse.BooleanOptionalAction)

    args = parser.parse_args()

    if args.output is None and not args.csv:
        parser.error("when plotting (a.k.a --no-csv), an output file name is required via --output")

    if args.output:
        dirs = os.path.dirname(args.output)

        if dirs:
            os.makedirs(dirs, exist_ok=True)

    simulate(args.output, plot=not args.csv)