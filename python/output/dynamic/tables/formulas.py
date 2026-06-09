# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

# region IMPORTS
from pyairtable import Table
from ...static.airtable_table import AirtableTable
from ..types import (
    FormulasField,
    FormulasCalculatedFields,
    FormulasCalculatedFieldIds,
    FormulasView,
    FormulasViewNameIdMapping,
    FormulasFields,
)
from ..dicts import (
    FormulasRecordDict,
    FormulasCreateRecordDict,
    FormulasUpdateRecordDict,
)
from ..models import FormulasModel
# endregion



# region FORMULAS
class FormulasTable(AirtableTable[FormulasRecordDict, FormulasCreateRecordDict, FormulasUpdateRecordDict, FormulasModel, FormulasView, FormulasField]):
    """
    An abstraction of pyAirtable's `Api.table` for the `Formulas` table, and an interface for working with custom-typed versions of the models/dicts created by the type generator.

    ```python
    record = Airtable().formulas.get("rec1234567890")
    ```

    You can also access the RecordDicts via `.dict`.
    
    ```python
    record = Airtable().formulas.dict.get("rec1234567890")
    ```

    You can also use the ORM Models directly. See https://pyairtable.readthedocs.io/en/stable/orm.html#
    """
    @classmethod
    def from_table(cls, table: Table, cache_seconds: int = 0):  # ty: ignore
        cls = super().from_table(
            table,
            FormulasRecordDict,  # ty: ignore[invalid-argument-type]
            FormulasCreateRecordDict,  # ty: ignore[invalid-argument-type]
            FormulasUpdateRecordDict,  # ty: ignore[invalid-argument-type]
            FormulasModel,  # ty: ignore[invalid-argument-type]
            FormulasCalculatedFields,
            FormulasCalculatedFieldIds,
            FormulasViewNameIdMapping,  # ty: ignore[invalid-argument-type]
            FormulasFields,  # ty: ignore[invalid-argument-type]
            cache_seconds=cache_seconds,
        )
        return cls
# endregion


