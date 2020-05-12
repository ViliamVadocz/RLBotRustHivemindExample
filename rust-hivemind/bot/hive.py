from rlbot.agents.hivemind.drone_agent import DroneAgent
from rlbot.agents.hivemind.subprocess_hivemind import SubprocessHivemind

from pathlib import Path

# Dummy agent to request the Rust Hivemind.
class Drone(DroneAgent):
    # Relative path to the hivemind BotHelperProcess python file.
    hive_path = __file__
    # Bots with the same key will be part of the same hivemind.
    hive_key = 'ThisShouldBeChangedSoItsUnique'
    # Name of your hivemind that shows up in the console.
    hive_name = 'ExampleHivemind'


class RustHivemind(SubprocessHivemind):
    # Path to the executable.
    exec_path = str(Path(__file__).parent.parent / 'target' / 'debug' / 'rust-example.exe')