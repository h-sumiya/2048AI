import tomllib
from pathlib import Path

ROOT = Path(__file__).parent
setting = tomllib.loads((ROOT / 'setting.toml').read_text())

episodes: int = setting["episodes"]

memory_size: int = setting["memory"]["size"]
memory_batch: int = setting["memory"]["batch"]

e_gamma: int = setting["e-greedy"]["gamma"]
e_start: int = setting["e-greedy"]["start"]
e_end: int = setting["e-greedy"]["end"]
e_decay: int = setting["e-greedy"]["decay"]

display_plot: int = setting["display"]["plot"]

save_time: int = setting["save"]["model"]
save_path: Path = ROOT / setting["save"]["path"]
save_path.parent.mkdir(parents=True, exist_ok=True)
