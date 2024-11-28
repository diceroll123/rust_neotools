from datetime import datetime

from rust_neotools import IslandMystic  # type: ignore  # noqa

USERNAME = "diceroll123"
CORRECT_ENGLISH_DATE = datetime(year=2022, month=10, day=30)
CORRECT_NON_ENGLISH_DATE = datetime(year=2030, month=6, day=21)
WRONG_DATE = datetime(year=2023, month=3, day=10)
HALLOWEEN_9999 = datetime(year=9999, month=10, day=30)


def test_mystic_english() -> None:
    assert IslandMystic.check(CORRECT_ENGLISH_DATE, USERNAME) is True


def test_mystic_english_wrong() -> None:
    assert IslandMystic.check(WRONG_DATE, USERNAME) is False


def test_mystic_non_english() -> None:
    assert IslandMystic.check_non_english(CORRECT_NON_ENGLISH_DATE, USERNAME) is True


def test_mystic_non_english_wrong() -> None:
    assert IslandMystic.check_non_english(WRONG_DATE, USERNAME) is False


def test_mystic_out_of_bounds_brute_force_user() -> None:
    assert IslandMystic.brute_force_user(HALLOWEEN_9999, USERNAME, 1, False) is None


def test_mystic_brute_force_user() -> None:
    assert IslandMystic.brute_force_user(
        datetime(year=2023, month=3, day=31), USERNAME, 1, True
    ) == datetime(year=2023, month=4, day=2)
