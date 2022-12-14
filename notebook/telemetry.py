"""
Parses and decodes telemetry data from a static-context stored in a file, or through a file stream
for live data-analysis (if the file is live, then the telemetry-accessing routine will yield
instead of throwing an error).
"""

import dataclasses
import enum
import typing
import os

import numpy as np
import quaternion
import struct

Vector3Field = dataclasses.field(default_factory=lambda: np.empty(3)),
SNAPSHOT_FORMAT = (
    "L"		# time: u64
    "I"		# state: u32
    "ddd"	# position: Vector3<f64>
    "dddd"	# orientation: Quaternion<f64>
    "ddd"	# velocity: Vector3<f64>
    "ddd"	# acceleration: Vector3<f64>
)


class SystemState(enum.Enum):
    """Rocket-state bitflag defenitions."""
    FINISHED = 0x03


@dataclasses.dataclass
class TelemetrySnapshot:
    """
    Reperesesnts sensor-data at a given instant.
    """
    time: int
    state: int
    position: Vector3Field
    orientation: dataclasses.field(default_factory=np.quaternion)
    velocity: Vector3Field
    accel: Vector3Field


def get_snapshot(stream: typing.BinaryIO) -> TelemetryShapshot:
    """Creates a TelemetryShapshot from a binary stream."""
    unpacked = struct.unpack(stream, SHAPSHOT_FORMAT)
    return TelemetrySnapshot(unpacked)


def snapshot_available(stream: typing.BinaryIO) -> bool:
    """Determines if another snapshot from the steam is available."""
    return os.fstat(stream).st_size >= struct.calcsize(SNAPSHOT_FORMAT)


def collect_snapshots(stream: typing.BinaryIO) -> list[TelemetryShapshot]:
    """Reduces stream of snapshots into an array. This is done by repeaetedly collecting new
    shapshots until the finish-state is reached."""
    snapshots = []

    while len(snapshots) < 1 or snapshot[len(snapshots - 1)].state != SystemState.FINISHED:
        snapshot = get_snapshot(stream)
        snapshots.append(snapshot)
    
    return shapshots
