# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

from typing import (
    TYPE_CHECKING,
    cast,
)
from pyairtable.orm import Model
from pyairtable.orm.fields import (
    SingleLineTextField,
    LinkField,
)
from ...static.helpers import (
    get_api_key,
    get_base_id,
    build_url,
)
from ..dicts import SecondaryRecordDict
from ..formulas import SecondaryFormulas
from ..types import (
    SecondaryView,
    SecondaryViewNameIdMapping,
)
if TYPE_CHECKING:
    from .tertiary import TertiaryModel
    from .primary import PrimaryModel


class SecondaryModel(Model):
    """
    ORM model for Airtable records from the `Secondary` table.

    Property names do not necessarily match field names in Airtable.
    """
    class Meta:
        @staticmethod
        def api_key() -> str:
            return get_api_key()
        @staticmethod
        def base_id() -> str:
            return get_base_id()
        table_name = "Secondary"
        use_field_ids = True
        memoize = True

    def to_record_dict(self, only_writable: bool = False) -> SecondaryRecordDict:
        return cast("SecondaryRecordDict", self.to_record(only_writable))

    def url(self, view: SecondaryView | None = None) -> str:
        """Get the URL for this record in Airtable, with optional view."""
        if view:
            return build_url(base_id=get_base_id(), table_id='tblPPScS3XMuFkDYN', record_id=self.id, view_id=SecondaryViewNameIdMapping[view])
        else:
            return build_url(base_id=get_base_id(), table_id='tblPPScS3XMuFkDYN', record_id=self.id)

    f: SecondaryFormulas = SecondaryFormulas()

    evaluate_formulas_at_runtime: bool = False

    link_to_tertiary: list["TertiaryModel"] = LinkField["TertiaryModel"](field_name="fldKR6tdbnOBRCtdQ", model="output.dynamic.models.tertiary.TertiaryModel")  # ty: ignore[invalid-assignment]
    """Link to Tertiary `fldKR6tdbnOBRCtdQ`"""
    name: SingleLineTextField = SingleLineTextField(field_name="fld1RagdJ09mpWhzM")
    """Name `fld1RagdJ09mpWhzM` - `Primary Key`"""
    primary: list["PrimaryModel"] = LinkField["PrimaryModel"](field_name="fldl0nB9WRFSdqlii", model="output.dynamic.models.primary.PrimaryModel")  # ty: ignore[invalid-assignment]
    """Primary `fldl0nB9WRFSdqlii`"""
    primary_2: list["PrimaryModel"] = LinkField["PrimaryModel"](field_name="fldgoE2oZmXmKkQca", model="output.dynamic.models.primary.PrimaryModel")  # ty: ignore[invalid-assignment]
    """Primary 2 `fldgoE2oZmXmKkQca`"""
    value: SingleLineTextField = SingleLineTextField(field_name="fldi6Mxh5H1gPGxFX")
    """Value `fldi6Mxh5H1gPGxFX`"""

