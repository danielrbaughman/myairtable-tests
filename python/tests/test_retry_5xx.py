"""Transient 5xx retry (Python) — the pyairtable default-config gap.

pyairtable retries 429 only by default (DEFAULT_RETRIABLE_STATUS_CODES = (429,)), so a 503
SERVICE_UNAVAILABLE surfaces immediately. The generated client constructs the ``Api`` with a
``retry_strategy`` that also covers transient 5xx. pyairtable performs the actual backoff (urllib3
Retry on the mounted HTTPAdapter), so this is a deterministic offline config assertion rather than a
live test: build the client with fake credentials (no network at construction) and inspect the
adapter's ``status_forcelist``.
"""

from output import Airtable


def test_api_retries_transient_5xx_not_just_429() -> None:
    # Use the real env creds (construction makes no network call). A fake base_id would register as
    # the module-level _default_base_id (helpers.set_airtable_config) and leak into later tests' ORM
    # models — this only inspects the retry config, so the live base is never actually hit.
    airtable = Airtable()
    adapter = airtable._api.session.get_adapter("https://api.airtable.com/")
    forcelist = set(adapter.max_retries.status_forcelist or ())

    # pyairtable's default is {429} only; we widen it to cover transient 5xx, incl. 503.
    assert 429 in forcelist
    assert {500, 502, 503, 504}.issubset(forcelist), f"expected 5xx in retry forcelist, got {sorted(forcelist)}"
