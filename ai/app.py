from setting import *
from py2048 import Board, Moves
from torch import nn
from collections import deque
from dataclasses import dataclass
from enum import Enum
import random
import torch


class Action(Enum):
    UP = 0
    DOWN = 1
    LEFT = 2
    RIGHT = 3


@dataclass
class Transition:
    state: Moves
    action: Action
    next_state: Moves
    reward: int = 1


class ReplayMemory:
    def __init__(self, capacity) -> None:
        self.memory = deque([], maxlen=capacity)

    def push(self, transition: Transition):
        self.memory.append(transition)

    def sample(self):
        return random.sample(self.memory, memory_batch)

    def __len__(self):
        return len(self.memory)


class DQN(nn.Module):
    def __init__(self) -> None:
        super(DQN, self).__init__()
        self.fc = nn.Sequential(
            nn.Linear(16, 16),
            nn.ReLU(),
            nn.Linear(16, 16),
            nn.ReLU(),
            nn.Linear(16, 8),
            nn.ReLU(),
            nn.Linear(8, 1)
        )

    def forward(self, up, down, left, right):
        res = [
            self.fc(up),
            self.fc(down),
            self.fc(left),
            self.fc(right)
        ]
        return torch.cat(res, dim=1)


model = DQN()
memory = ReplayMemory(memory_size)
