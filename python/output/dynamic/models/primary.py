# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

from datetime import datetime, timedelta
from typing import Any, TYPE_CHECKING

from pyairtable.orm import Model
from pyairtable.orm.fields import SingleLineTextField, MultilineTextField, PhoneNumberField, EmailField, LinkField, SingleLinkField, UrlField, DateField, CreatedTimeField, LastModifiedTimeField, NumberField, SelectField, MultipleSelectField, CheckboxField, RichTextField, CurrencyField, PercentField, LookupField, AttachmentsField, CreatedByField, ButtonField, CountField, DatetimeField, DurationField, LastModifiedByField, AutoNumberField, CollaboratorField, MultipleCollaboratorsField

from ...static.helpers import get_api_key, get_base_id
from ...static.special_types import AirtableAttachment, RecordId
from ...static.airtable_runtime import AirtableRuntime as F
import urllib.parse
import math
import re
from ..types import (
    PrimaryMultipleSelectOption,
    PrimarySingleSelectOption,
)
from ..dicts import PrimaryRecordDict
from ..formulas import PrimaryFormulas
from ..options import PrimaryOptions
if TYPE_CHECKING:
    from .secondary import SecondaryModel


class PrimaryModel(Model):
    """
    ORM model for Airtable records from the `Primary` table.

    Property names do not necessarily match field names in Airtable.
    """
    class Meta:
        @staticmethod
        def api_key() -> str:
            return get_api_key()
        @staticmethod
        def base_id() -> str:
            return get_base_id()
        table_name = "Primary"
        use_field_ids = True
        memoize = True

    def to_record_dict(self) -> PrimaryRecordDict:
        return self.to_record()

    f: PrimaryFormulas = PrimaryFormulas()

    o: PrimaryOptions = PrimaryOptions()

    evaluate_formulas_at_runtime: bool = False

    attachment: AttachmentsField = AttachmentsField(field_name="fldhF2AEuSC1haCZd")
    """Attachment `fldhF2AEuSC1haCZd`"""
    auto_number: AutoNumberField = AutoNumberField(field_name="fldizvTkxgIn0mC3L", readonly=True)
    """Auto Number `fldizvTkxgIn0mC3L` - `Read-Only Field`"""
    button: ButtonField = ButtonField(field_name="fldY48yKPG16AajtU", readonly=True)
    """Button `fldY48yKPG16AajtU` - `Read-Only Field`"""
    checkbox: CheckboxField = CheckboxField(field_name="fldjQIaAZVegb1FUa")
    """Checkbox `fldjQIaAZVegb1FUa`"""
    created_by: CreatedByField = CreatedByField(field_name="fldGLQhDz2UjjiHG6", readonly=True)
    """Created By `fldGLQhDz2UjjiHG6` - `Read-Only Field`"""
    created_at_time: CreatedTimeField = CreatedTimeField(field_name="fld2YgW382Kt9xltA", readonly=True)
    """Created Time `fld2YgW382Kt9xltA` - `Read-Only Field`"""
    currency_float: CurrencyField = CurrencyField(field_name="fldyh8pzDXiy5abEr")
    """Currency (float) `fldyh8pzDXiy5abEr`"""
    currency_int: CurrencyField = CurrencyField(field_name="fldBfo74z9hD78hP8")
    """Currency (int) `fldBfo74z9hD78hP8`"""
    date: DateField = DateField(field_name="fldC6LfNVvVIxKyQH")
    """Date `fldC6LfNVvVIxKyQH`"""
    date_with_time: DatetimeField = DatetimeField(field_name="fldizYmjpXABGDLTG")
    """Date (with time) `fldizYmjpXABGDLTG`"""
    duration: DurationField = DurationField(field_name="fldLTyf6ljS0rhur8")
    """Duration `fldLTyf6ljS0rhur8`"""
    email: EmailField = EmailField(field_name="fldHCJoYBiFVsNvP4")
    """Email `fldHCJoYBiFVsNvP4`"""
    _orm_formula_complex: SingleLineTextField = SingleLineTextField(field_name="fld2vnFc0Bl5IOFUQ", readonly=True)
    @property
    def formula_complex(self) -> str | None:
        """
        Formula (Complex) `fld2vnFc0Bl5IOFUQ` - `Read-Only Field`
        
        ```
        CONCATENATE(
          "Primary Key: ",
          {Primary Key},
          "\n",
          "Single Line Text: ",
          {Single Line Text},
          "\n",
          "Long Text: ",
          {Long Text},
          "\n",
          "Long Text with Rich Text: ",
          {Long Text with Rich Text},
          "\n",
          "Attachment: ",
          IF(
            {Attachment},
            {Attachment},
            "None"
          ),
          "\n",
          "Checkbox: ",
          IF(
            {Checkbox},
            "Checked",
            "Unchecked"
          ),
          "\n",
          "Multiple Select: ",
          IF(
            {Multiple Select},
            {Multiple Select},
            "None"
          ),
          "\n",
          "Single Select: ",
          IF(
            {Single Select},
            {Single Select},
            "None"
          ),
          "\n",
          "User: ",
          IF(
            {User},
            {User},
            "None"
          ),
          "\n",
          "User (allow multiple): ",
          IF(
            {User (allow multiple)},
            {User (allow multiple)},
            "None"
          ),
          "\n",
          "Date: ",
          IF(
            {Date},
            DATETIME_FORMAT({Date}, 'YYYY-MM-DD'),
            "None"
          ),
          "\n",
          "Date (with time): ",
          IF(
            {Date (with time)},
            DATETIME_FORMAT(
              {Date (with time)},
              'YYYY-MM-DD HH:mm'
            ),
            "None"
          ),
          "\n",
          "Phone Number: ",
          IF(
            {Phone Number},
            {Phone Number},
            "None"
          ),
          "\n",
          "Email: ",
          IF(
            {Email},
            {Email},
            "None"
          ),
          "\n",
          "URL: ",
          IF(
            {URL},
            {URL},
            "None"
          ),
          "\n",
          "Number (int): ",
          IF(
            {Number (int)},
            {Number(int) } & "",
            "None"
          ),
          "\n",
          "Number (float): ",
          IF(
            {Number (float)},
            {Number(float) } & "",
            "None"
          ),
          "\n",
          "Currency (int): ",
          IF(
            {Currency (int)},
            {Currency(int) } & "",
            "None"
          ),
          "\n",
          "Currency (float): ",
          IF(
            {Currency (float)},
            {Currency(float) } & "",
            "None"
          ),
          "\n",
          "Percent (int): ",
          IF(
            {Percent (int)},
            {Percent(int) } & "",
            "None"
          ),
          "\n",
          "Percent (float): ",
          IF(
            {Percent (float)},
            {Percent(float) } & "",
            "None"
          ),
          "\n",
          "Duration: ",
          IF(
            {Duration},
            {Duration} & "",
            "None"
          ),
          "\n",
          "Rating: ",
          IF(
            {Rating},
            {Rating} & "",
            "None"
          ),
          "\n",
          "Created Time: ",
          IF(
            {Created Time},
            DATETIME_FORMAT(
              {Created Time},
              'YYYY-MM-DD HH:mm'
            ),
            "None"
          ),
          "\n",
          "Last Modified Time: ",
          IF(
            {Last Modified Time},
            DATETIME_FORMAT(
              {Last Modified Time},
              'YYYY-MM-DD HH:mm'
            ),
            "None"
          ),
          "\n",
          "Created By: ",
          IF(
            {Created By},
            {Created By},
            "None"
          ),
          "\n",
          "Last Modified By: ",
          IF(
            {Last Modified By},
            {Last Modified By},
            "None"
          ),
          "\n",
          "Auto Number: ",
          IF(
            {Auto Number},
            {Auto Number} & "",
            "None"
          ),
          "\n",
          "Button: ",
          IF(
            {Button},
            {Button},
            "None"
          ),
          "\n",
          "Link (single): ",
          IF(
            {Link (single)},
            {Link (single)},
            "None"
          ),
          "\n",
          "Link (multiple): ",
          IF(
            {Link (multiple)},
            {Link (multiple)},
            "None"
          ),
          "\n",
          "Lookup: ",
          IF(
            {Lookup},
            {Lookup},
            "None"
          ),
          "\n",
          "Rollup: ",
          IF(
            {Rollup},
            {Rollup},
            "None"
          ),
          "\n",
          "Formula (ID): ",
          IF(
            {Formula (ID)},
            {Formula (ID)},
            "None"
          ),
          "\n",
          "Formula (Simple): ",
          IF(
            {Formula (Simple)},
            {Formula (Simple)},
            "None"
          )
        )
        ```
        """
        if self.evaluate_formulas_at_runtime:
            self._fields["fld2vnFc0Bl5IOFUQ"] = "".join(F.AS(("Primary Key: ", self.primary_key, "\n", "Single Line Text: ", self.single_line_text, "\n", "Long Text: ", self.long_text, "\n", "Long Text with Rich Text: ", self.long_text_with_rich_text, "\n", "Attachment: ", (self.attachment if self.attachment else "None"), "\n", "Checkbox: ", ("Checked" if self.checkbox else "Unchecked"), "\n", "Multiple Select: ", (self.multiple_select if self.multiple_select else "None"), "\n", "Single Select: ", (self.single_select if self.single_select else "None"), "\n", "User: ", (self.user if self.user else "None"), "\n", "User (allow multiple): ", (self.user_allow_multiple if self.user_allow_multiple else "None"), "\n", "Date: ", (F.DATETIME_FORMAT(self.date, 'YYYY-MM-DD') if self.date else "None"), "\n", "Date (with time): ", (F.DATETIME_FORMAT(self.date_with_time, 'YYYY-MM-DD HH:mm') if self.date_with_time else "None"), "\n", "Phone Number: ", (self.phone_number if self.phone_number else "None"), "\n", "Email: ", (self.email if self.email else "None"), "\n", "URL: ", (self.url if self.url else "None"), "\n", "Number (int): ", ((F.S(self.number_int) + "") if self.number_int else "None"), "\n", "Number (float): ", ((F.S(self.number_float) + "") if self.number_float else "None"), "\n", "Currency (int): ", ((F.S(self.currency_int) + "") if self.currency_int else "None"), "\n", "Currency (float): ", ((F.S(self.currency_float) + "") if self.currency_float else "None"), "\n", "Percent (int): ", ((F.S(self.percent_int) + "") if self.percent_int else "None"), "\n", "Percent (float): ", ((F.S(self.percent_float) + "") if self.percent_float else "None"), "\n", "Duration: ", ((F.S(self.duration) + "") if self.duration else "None"), "\n", "Rating: ", ((F.S(self.rating) + "") if self.rating else "None"), "\n", "Created Time: ", (F.DATETIME_FORMAT(self.created_at_time, 'YYYY-MM-DD HH:mm') if self.created_at_time else "None"), "\n", "Last Modified Time: ", (F.DATETIME_FORMAT(self.last_modified_time, 'YYYY-MM-DD HH:mm') if self.last_modified_time else "None"), "\n", "Created By: ", (self.created_by if self.created_by else "None"), "\n", "Last Modified By: ", (self.last_modified_by if self.last_modified_by else "None"), "\n", "Auto Number: ", ((F.S(self.auto_number) + "") if self.auto_number else "None"), "\n", "Button: ", (self.button if self.button else "None"), "\n", "Link (single): ", (self.link_single if self.link_single else "None"), "\n", "Link (multiple): ", (self.link_multiple if self.link_multiple else "None"), "\n", "Lookup: ", (self.lookup if self.lookup else "None"), "\n", "Rollup: ", (self.rollup if self.rollup else "None"), "\n", "Formula (ID): ", (self.formula_id if self.formula_id else "None"), "\n", "Formula (Simple): ", (self.formula_simple if self.formula_simple else "None"),)))
        return self._fields.get("fld2vnFc0Bl5IOFUQ")
    _orm_formula_id: SingleLineTextField = SingleLineTextField(field_name="fldcf62YFeIIDHElt", readonly=True)
    @property
    def formula_id(self) -> str | None:
        """
        Formula (ID) `fldcf62YFeIIDHElt` - `Read-Only Field`
        
        ```
        RECORD_ID()
        ```
        """
        if self.evaluate_formulas_at_runtime:
            self._fields["fldcf62YFeIIDHElt"] = self.id
        return self._fields.get("fldcf62YFeIIDHElt")
    _orm_formula_nested: SingleLineTextField = SingleLineTextField(field_name="fldXFeHRPBLz6AiWh", readonly=True)
    @property
    def formula_nested(self) -> str | None:
        """
        Formula (Nested) `fldXFeHRPBLz6AiWh` - `Read-Only Field`
        
        ```
        {Formula (ID)} & {Formula (Simple)} & {Formula (Complex)}
        ```
        """
        if self.evaluate_formulas_at_runtime:
            self._fields["fldXFeHRPBLz6AiWh"] = ((F.S(self.formula_id) + F.S(self.formula_simple)) + F.S(self.formula_complex))
        return self._fields.get("fldXFeHRPBLz6AiWh")
    _orm_formula_simple: NumberField = NumberField(field_name="fldy1axxaoUToLVC6", readonly=True)
    @property
    def formula_simple(self) -> float | None:
        """
        Formula (Simple) `fldy1axxaoUToLVC6` - `Read-Only Field`
        
        ```
        {Number (int)} + {Number (float)}
        ```
        """
        if self.evaluate_formulas_at_runtime:
            self._fields["fldy1axxaoUToLVC6"] = (F.N(self.number_int) + F.N(self.number_float))
        return self._fields.get("fldy1axxaoUToLVC6")
    last_modified_by: LastModifiedByField = LastModifiedByField(field_name="fldF8iDttqP0AgzWC", readonly=True)
    """Last Modified By `fldF8iDttqP0AgzWC` - `Read-Only Field`"""
    last_modified_time: LastModifiedTimeField = LastModifiedTimeField(field_name="fldMinKh4pa3YX86g", readonly=True)
    """Last Modified Time `fldMinKh4pa3YX86g` - `Read-Only Field`"""
    link_multiple: list["SecondaryModel"] = LinkField["SecondaryModel"](field_name="fldFyFheQWczd8oux", model="output.dynamic.models.secondary.SecondaryModel") # type: ignore
    """Link (multiple) `fldFyFheQWczd8oux`"""
    link_single: "SecondaryModel" = SingleLinkField["SecondaryModel"](field_name="fld7F5onkDo6mkmbN", model="output.dynamic.models.secondary.SecondaryModel") # type: ignore
    """Link (single) `fld7F5onkDo6mkmbN`"""
    long_text: MultilineTextField = MultilineTextField(field_name="fld8ulc6J0W29M6La")
    """Long Text `fld8ulc6J0W29M6La`"""
    long_text_with_rich_text: MultilineTextField = MultilineTextField(field_name="fldHJkxCMC0xo343u")
    """Long Text with Rich Text `fldHJkxCMC0xo343u`"""
    lookup: LookupField = LookupField[list[str]](field_name="fldbmFmrzYKBktJvE", readonly=True)
    """Lookup `fldbmFmrzYKBktJvE` - `Read-Only Field`"""
    multiple_select: list[PrimaryMultipleSelectOption] = MultipleSelectField(field_name="fld6GTabFmu1xKPvZ") # type: ignore
    """Multiple Select `fld6GTabFmu1xKPvZ`"""
    number_float: NumberField = NumberField(field_name="fldmU0X2l4RWd21dd")
    """Number (float) `fldmU0X2l4RWd21dd`"""
    number_int: NumberField = NumberField(field_name="fldOfPKGmnRPv94QH")
    """Number (int) `fldOfPKGmnRPv94QH`"""
    percent_float: PercentField = PercentField(field_name="fldiGui9ll69N7WOj")
    """Percent (float) `fldiGui9ll69N7WOj`"""
    percent_int: PercentField = PercentField(field_name="fldbAAyWboGulpb4s")
    """Percent (int) `fldbAAyWboGulpb4s`"""
    phone_number: PhoneNumberField = PhoneNumberField(field_name="fld38tnNpHmoks8C8")
    """Phone Number `fld38tnNpHmoks8C8`"""
    primary_key: SingleLineTextField = SingleLineTextField(field_name="fldol5Q4wmQJQvPRy")
    """Primary Key `fldol5Q4wmQJQvPRy` - `Primary Key`"""
    rating: Any
    """Rating `fldRsmwFwQNZkKLp4`"""
    rollup: SingleLineTextField = SingleLineTextField(field_name="fldGaFgBsDC3IBUdV", readonly=True)
    """Rollup `fldGaFgBsDC3IBUdV` - `Read-Only Field`"""
    single_line_text: SingleLineTextField = SingleLineTextField(field_name="fld0BL2lFo9fqcKv3")
    """Single Line Text `fld0BL2lFo9fqcKv3`"""
    single_select: PrimarySingleSelectOption = SelectField(field_name="fldn0GFFtMFpCXUNU")
    """Single Select `fldn0GFFtMFpCXUNU`"""
    url: UrlField = UrlField(field_name="fldLYloz2oP4ymf3B")
    """URL `fldLYloz2oP4ymf3B`"""
    user: CollaboratorField = CollaboratorField(field_name="fldU6SbLp8CSkLcA4")
    """User `fldU6SbLp8CSkLcA4`"""
    user_allow_multiple: MultipleCollaboratorsField = MultipleCollaboratorsField(field_name="fldBwCDbAVxRj9yg7")
    """User (allow multiple) `fldBwCDbAVxRj9yg7`"""

