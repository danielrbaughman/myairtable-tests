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
from ..dicts import TertiaryRecordDict
from ..formulas import TertiaryFormulas
from ..types import (
    TertiaryView,
    TertiaryViewNameIdMapping,
)
if TYPE_CHECKING:
    from .secondary import SecondaryModel


class TertiaryModel(Model):
    """
    ORM model for Airtable records from the `Tertiary` table.

    Property names do not necessarily match field names in Airtable.
    """
    class Meta:
        @staticmethod
        def api_key() -> str:
            return get_api_key()
        @staticmethod
        def base_id() -> str:
            return get_base_id()
        table_name = "Tertiary"
        use_field_ids = True
        memoize = True

    def to_record_dict(self, only_writable: bool = False) -> TertiaryRecordDict:
        return cast("TertiaryRecordDict", self.to_record(only_writable))

    def url(self, view: TertiaryView | None = None) -> str:
        """Get the URL for this record in Airtable, with optional view."""
        if view:
            return build_url(base_id=get_base_id(), table_id='tblLFoLxEdWlxjmLP', record_id=self.id, view_id=TertiaryViewNameIdMapping[view])
        else:
            return build_url(base_id=get_base_id(), table_id='tblLFoLxEdWlxjmLP', record_id=self.id)

    f: TertiaryFormulas = TertiaryFormulas()

    evaluate_formulas_at_runtime: bool = False

    name: SingleLineTextField = SingleLineTextField(field_name="fldwzqKxsRnPZJ2Ll")
    """Name `fldwzqKxsRnPZJ2Ll` - `Primary Key`"""
    secondary: list["SecondaryModel"] = LinkField["SecondaryModel"](field_name="fld8lCuUXpEXkIeYv", model="output.dynamic.models.secondary.SecondaryModel")  # ty: ignore[invalid-assignment]
    """Secondary `fld8lCuUXpEXkIeYv`"""
    value: SingleLineTextField = SingleLineTextField(field_name="fldjNLBh2UccM64h5")
    """Value `fldjNLBh2UccM64h5`"""

