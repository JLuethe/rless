"""Test-only deterministic fault/handshake primitives; not product APIs."""
import os
import select


class FaultPlan:
    def __init__(self, fail_at=None):
        self.fail_at = fail_at
        self.trace = []

    def checkpoint(self, name):
        self.trace.append(name)
        if self.fail_at == len(self.trace):
            raise MemoryError('injected at ' + name)


def transactional_append(storage, data, faults):
    """Ownership/rollback model only; not coverage of the translated buffer."""
    faults.checkpoint('reserve')
    candidate = bytearray(storage)
    candidate.extend(data)
    faults.checkpoint('commit')
    storage[:] = candidate


class Barrier:
    """A real pipe readiness handshake, with a bounded wait and explicit close."""
    def __init__(self):
        self.read_fd, self.write_fd = os.pipe2(os.O_CLOEXEC)

    def ready(self):
        os.write(self.write_fd, b'R')

    def wait(self, timeout=5):
        if not select.select([self.read_fd], [], [], timeout)[0]:
            raise TimeoutError('fixture readiness not observed')
        if os.read(self.read_fd, 1) != b'R':
            raise RuntimeError('invalid fixture handshake')

    def close(self):
        for fd in (self.read_fd, self.write_fd):
            os.close(fd)

    def __enter__(self):
        return self

    def __exit__(self, *_):
        self.close()
