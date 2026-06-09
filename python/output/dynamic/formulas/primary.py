# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

from ...static.formula import (
    AttachmentsField,
    BooleanField,
    DateField,
    LookupField,
    NumberField,
    TextField,
    SingleSelectField,
    MultiSelectField,
    ID,
)
from ..types import (
    PrimaryMultipleSelectOption,
    PrimarySingleSelectOption,
)

# region PROPERTIES
class PrimaryFormulas:
    id: ID = ID()
    attachment: AttachmentsField = AttachmentsField('fldhF2AEuSC1haCZd')
    """Attachment `fldhF2AEuSC1haCZd`"""
    auto_number: NumberField = NumberField('fldizvTkxgIn0mC3L')
    """Auto Number `fldizvTkxgIn0mC3L` - `Read-Only Field`"""
    button: TextField = TextField('fldY48yKPG16AajtU')
    """Button `fldY48yKPG16AajtU` - `Read-Only Field`"""
    checkbox: BooleanField = BooleanField('fldjQIaAZVegb1FUa')
    """Checkbox `fldjQIaAZVegb1FUa`"""
    created_by: TextField = TextField('fldGLQhDz2UjjiHG6')
    """Created By `fldGLQhDz2UjjiHG6` - `Read-Only Field`"""
    created_at_time: DateField = DateField('fld2YgW382Kt9xltA')
    """Created Time `fld2YgW382Kt9xltA` - `Read-Only Field`"""
    currency_float: NumberField = NumberField('fldyh8pzDXiy5abEr')
    """Currency (float) `fldyh8pzDXiy5abEr`"""
    currency_int: NumberField = NumberField('fldBfo74z9hD78hP8')
    """Currency (int) `fldBfo74z9hD78hP8`"""
    date: DateField = DateField('fldC6LfNVvVIxKyQH')
    """Date `fldC6LfNVvVIxKyQH`"""
    date_with_time: DateField = DateField('fldizYmjpXABGDLTG')
    """Date (with time) `fldizYmjpXABGDLTG`"""
    duration: NumberField = NumberField('fldLTyf6ljS0rhur8')
    """Duration `fldLTyf6ljS0rhur8`"""
    email: TextField = TextField('fldHCJoYBiFVsNvP4')
    """Email `fldHCJoYBiFVsNvP4`"""
    formula_complex: TextField = TextField('fld2vnFc0Bl5IOFUQ')
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
    formula_id: TextField = TextField('fldcf62YFeIIDHElt')
    """
    Formula (ID) `fldcf62YFeIIDHElt` - `Read-Only Field`
    
    ```
    RECORD_ID()
    ```
    """
    formula_nested: TextField = TextField('fldXFeHRPBLz6AiWh')
    """
    Formula (Nested) `fldXFeHRPBLz6AiWh` - `Read-Only Field`
    
    ```
    {Formula (ID)} & {Formula (Simple)} & {Formula (Complex)}
    ```
    """
    formula_simple: NumberField = NumberField('fldy1axxaoUToLVC6')
    """
    Formula (Simple) `fldy1axxaoUToLVC6` - `Read-Only Field`
    
    ```
    {Number (int)} + {Number (float)}
    ```
    """
    last_modified_by: TextField = TextField('fldF8iDttqP0AgzWC')
    """Last Modified By `fldF8iDttqP0AgzWC` - `Read-Only Field`"""
    last_modified_time: DateField = DateField('fldMinKh4pa3YX86g')
    """Last Modified Time `fldMinKh4pa3YX86g` - `Read-Only Field`"""
    link_multiple: TextField = TextField('fldFyFheQWczd8oux')
    """Link (multiple) `fldFyFheQWczd8oux`"""
    link_single: TextField = TextField('fld7F5onkDo6mkmbN')
    """Link (single) `fld7F5onkDo6mkmbN`"""
    long_text: TextField = TextField('fld8ulc6J0W29M6La')
    """Long Text `fld8ulc6J0W29M6La`"""
    long_text_with_rich_text: TextField = TextField('fldHJkxCMC0xo343u')
    """Long Text with Rich Text `fldHJkxCMC0xo343u`"""
    lookup: LookupField = LookupField('fldbmFmrzYKBktJvE')
    """Lookup `fldbmFmrzYKBktJvE` - `Read-Only Field`"""
    multiple_select: MultiSelectField[PrimaryMultipleSelectOption] = MultiSelectField('fld6GTabFmu1xKPvZ')
    """Multiple Select `fld6GTabFmu1xKPvZ`"""
    number_float: NumberField = NumberField('fldmU0X2l4RWd21dd')
    """Number (float) `fldmU0X2l4RWd21dd`"""
    number_int: NumberField = NumberField('fldOfPKGmnRPv94QH')
    """Number (int) `fldOfPKGmnRPv94QH`"""
    percent_float: NumberField = NumberField('fldiGui9ll69N7WOj')
    """Percent (float) `fldiGui9ll69N7WOj`"""
    percent_int: NumberField = NumberField('fldbAAyWboGulpb4s')
    """Percent (int) `fldbAAyWboGulpb4s`"""
    phone_number: TextField = TextField('fld38tnNpHmoks8C8')
    """Phone Number `fld38tnNpHmoks8C8`"""
    primary_key: TextField = TextField('fldol5Q4wmQJQvPRy')
    """Primary Key `fldol5Q4wmQJQvPRy` - `Primary Key`"""
    rating: TextField = TextField('fldRsmwFwQNZkKLp4')
    """Rating `fldRsmwFwQNZkKLp4`"""
    rollup: TextField = TextField('fldGaFgBsDC3IBUdV')
    """Rollup `fldGaFgBsDC3IBUdV` - `Read-Only Field`"""
    single_line_text: TextField = TextField('fld0BL2lFo9fqcKv3')
    """Single Line Text `fld0BL2lFo9fqcKv3`"""
    single_select: SingleSelectField[PrimarySingleSelectOption] = SingleSelectField('fldn0GFFtMFpCXUNU')
    """Single Select `fldn0GFFtMFpCXUNU`"""
    url: TextField = TextField('fldLYloz2oP4ymf3B')
    """URL `fldLYloz2oP4ymf3B`"""
    user: TextField = TextField('fldU6SbLp8CSkLcA4')
    """User `fldU6SbLp8CSkLcA4`"""
    user_allow_multiple: TextField = TextField('fldBwCDbAVxRj9yg7')
    """User (allow multiple) `fldBwCDbAVxRj9yg7`"""

# endregion

