# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

# region IMPORTS
from typing import cast
from pyairtable import (
    Api,
    retry_strategy,
)
from .types import TableName
from ..static.airtable_table import AirtableTable
from ..static.helpers import (
    get_api_key,
    get_base_id,
    set_airtable_config,
    build_url,
)
from ..static.schema_types import BaseSchema
from .tables import (
    FormulasTable,
    PrimaryTable,
    SecondaryTable,
    TertiaryTable,
)
# endregion



# region MAIN CLASS
class Airtable:
    """
    A collection of tables abstracting pyAirtable's `Api.table`. Represents the whole Airtable base.
    
    Provides an interface for working with custom-typed versions of the models/dicts created by the type generator, for each of the tables in the Airtable base.

    ```python
    record = Airtable().tablename.get("rec1234567890")
    ```

    You can also access the RecordDicts via `.dict`.
    
    ```python
    record = Airtable().tablename.dict.get("rec1234567890")
    ```

    You can also use the ORM Models directly. See https://pyairtable.readthedocs.io/en/stable/orm.html#
    """

    _api: Api
    base_id: str
    _tables: dict[TableName, AirtableTable] = {}

    def __init__(self, api_key: str | None = None, base_id: str | None = None, endpoint_url: str = "https://api.airtable.com", cache_seconds: int = 0):
        self.base_id: str = base_id or get_base_id()
        if not self.base_id:
            raise ValueError("Base ID must be provided.")
        api_key: str = api_key or get_api_key()
        if not api_key:
            raise ValueError("API key must be provided.")
        # Register config so ORM models can look it up
        set_airtable_config(self.base_id, api_key, endpoint_url)
        self._cache_seconds: int = cache_seconds
        # pyairtable retries 429 only by default; also retry transient 5xx (incl. 503).
        # allowed_methods excludes POST so create (POST) is never retried (urllib3 is method-based,
        # so it can't distinguish idempotent from non-idempotent at the body level). Residuals:
        # (1) PATCH upsert-without-merge is still retried (can't be told apart from update-by-id);
        # (2) POST no longer retries even on 429 (safe: 429 means the request was rejected, nothing applied).
        self._api = Api(api_key=api_key, endpoint_url=endpoint_url, retry_strategy=retry_strategy(status_forcelist=(429, 500, 502, 503, 504), allowed_methods=frozenset({"GET", "HEAD", "OPTIONS", "TRACE", "PUT", "DELETE", "PATCH"})))

    def table(self, table_name: TableName) -> AirtableTable:
        """Get a table by its Airtable name."""
        from .types import TableNamePropertyMapping
        return getattr(self, TableNamePropertyMapping[table_name])

    def url(self) -> str:
        """Get the URL for the Airtable base."""
        return build_url(base_id=self.base_id)

    def get_schema(self) -> BaseSchema:
        """Fetch a live version of the schema from Airtable's metadata API."""
        import json
        import urllib.request
        url = f"https://api.airtable.com/v0/meta/bases/{self.base_id}/tables"
        req = urllib.request.Request(url, headers={"Authorization": f"Bearer {get_api_key(self.base_id)}"})
        with urllib.request.urlopen(req) as resp:
            return json.loads(resp.read())

    def invalidate_cache(self) -> None:
        """Invalidates the cache for all tables."""
        for table in self._tables.values():
            table.invalidate_cache()

    @property
    def formulas(self) -> FormulasTable:
        """`Formulas` (tblnuYBsMdXNDsuRc)"""
        if 'Formulas' not in self._tables:
            self._tables["Formulas"] = FormulasTable.from_table(self._api.table(self.base_id, "tblnuYBsMdXNDsuRc"), cache_seconds=self._cache_seconds)
        return cast("FormulasTable", self._tables["Formulas"])

    @property
    def primary(self) -> PrimaryTable:
        """`Primary` (tblmb3iqgpNS1ysV2)"""
        if 'Primary' not in self._tables:
            self._tables["Primary"] = PrimaryTable.from_table(self._api.table(self.base_id, "tblmb3iqgpNS1ysV2"), cache_seconds=self._cache_seconds)
        return cast("PrimaryTable", self._tables["Primary"])

    @property
    def secondary(self) -> SecondaryTable:
        """`Secondary` (tblPPScS3XMuFkDYN)"""
        if 'Secondary' not in self._tables:
            self._tables["Secondary"] = SecondaryTable.from_table(self._api.table(self.base_id, "tblPPScS3XMuFkDYN"), cache_seconds=self._cache_seconds)
        return cast("SecondaryTable", self._tables["Secondary"])

    @property
    def tertiary(self) -> TertiaryTable:
        """`Tertiary` (tblLFoLxEdWlxjmLP)"""
        if 'Tertiary' not in self._tables:
            self._tables["Tertiary"] = TertiaryTable.from_table(self._api.table(self.base_id, "tblLFoLxEdWlxjmLP"), cache_seconds=self._cache_seconds)
        return cast("TertiaryTable", self._tables["Tertiary"])

# endregion

