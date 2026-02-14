# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

# region IMPORTS
from pyairtable import Api

from .types import TableName
from ..static.airtable_table import TableType
from ..static.helpers import get_api_key, get_base_id, set_airtable_config
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
    _base_id: str
    _tables: dict[TableName, TableType] = {}

    def __init__(self, api_key: str | None = None, base_id: str | None = None, endpoint_url: str = "https://api.airtable.com"):
        self._base_id: str = base_id or get_base_id()
        if not self._base_id:
            raise ValueError("Base ID must be provided.")
        api_key: str = api_key or get_api_key()
        if not api_key:
            raise ValueError("API key must be provided.")
        # Register config so ORM models can look it up
        set_airtable_config(self._base_id, api_key, endpoint_url)
        self._api = Api(api_key=api_key, endpoint_url=endpoint_url)

    @property
    def formulas(self) -> FormulasTable:
        """`Formulas` (tblnuYBsMdXNDsuRc)"""
        if 'Formulas' not in self._tables:
            self._tables["Formulas"] = FormulasTable.from_table(self._api.table(self._base_id, "Formulas"))
        return self._tables["Formulas"]

    @property
    def primary(self) -> PrimaryTable:
        """`Primary` (tblmb3iqgpNS1ysV2)"""
        if 'Primary' not in self._tables:
            self._tables["Primary"] = PrimaryTable.from_table(self._api.table(self._base_id, "Primary"))
        return self._tables["Primary"]

    @property
    def secondary(self) -> SecondaryTable:
        """`Secondary` (tblPPScS3XMuFkDYN)"""
        if 'Secondary' not in self._tables:
            self._tables["Secondary"] = SecondaryTable.from_table(self._api.table(self._base_id, "Secondary"))
        return self._tables["Secondary"]

    @property
    def tertiary(self) -> TertiaryTable:
        """`Tertiary` (tblLFoLxEdWlxjmLP)"""
        if 'Tertiary' not in self._tables:
            self._tables["Tertiary"] = TertiaryTable.from_table(self._api.table(self._base_id, "Tertiary"))
        return self._tables["Tertiary"]

# endregion

