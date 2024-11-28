from datetime import datetime

from rust_neotools import Symol  # type: ignore  # noqa


def test_symol_minute() -> None:
    assert Symol.get_minute(datetime(year=2023, month=3, day=10)) == 9


def test_symol_minute_skip_day() -> None:
    assert Symol.get_minute(datetime(year=2022, month=9, day=19)) == 60


def test_symol_window() -> None:
    assert Symol.get_window(datetime(year=2023, month=3, day=10)) == [9, 10, 11, 12]


def test_symol_window_skip_day_capped() -> None:
    assert Symol.get_window(datetime(year=2023, month=4, day=5)) == [58, 59]


def test_symol_window_skip_day() -> None:
    assert Symol.get_window(datetime(year=2022, month=9, day=19)) == []
