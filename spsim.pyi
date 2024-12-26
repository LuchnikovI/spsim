from typing import List, Tuple

class SPSim:
    """Creates a sparce pauli simulator of quantum circuits.
    Args:
        qubits_number: number of qubits in a simulator.
    Notes:
        Once created, the number of qubits can be > `qubits_number`.
        This is due to the internal implementation of paulis strings.
    """
    def __init__(self, qubits_number: int) -> None: ...
    """Adds a gate to a layer of the simulator of the following form exp(i * time * pauli_string).
    Args:
        pauli_string_discreption: discription of a pauli string defining a gate,
            i.e. a list of pairs of pauli matrix literals ('X', 'Y', 'Z') and their positions;
        time: time in the gate expression exp(i * time * pauli_string)."""
    def add_gate(
        self,
        pauli_string_discreption: List[Tuple[str, int]],
        time: float,
    ) -> None: ...
    """Returns number of qubits in a simulator."""
    def qubits_number(self) -> int: ...
    """Evaluates dynamics of a given observable.
    Args:
        observable_description: discription of a pauli string defining an observable,
            i.e. a list of pairs of pauli matrix literals ('X', 'Y', 'Z') and their positions;
        layers_number: number of layers (discrete time steps);
        threshold and decay: define which pauli strings are being truncated during evolution as follows:
            if ampl(pauli_string) / 2 ** (decay * hamming(pauli_string) / 2) < threshold its being truncated.
    Returns:
        list with time dynamics of an observable."""
    def execute(
        self,
        observable_description: List[Tuple[str, int]],
        layers_number: int,
        threshold: float,
        decay: float = 1.,
    ) -> List[float]: ...