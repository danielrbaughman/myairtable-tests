# ==========================================
# Auto-generated file. Do not edit directly.
# ==========================================

from ...static.formula import (
    DateField,
    NumberField,
    TextField,
    ID,
)

# region PROPERTIES
class FormulasFormulas:
    id: ID = ID()
    date_formula: TextField = TextField('fldY7kjaklLeoSgGd')
    """
    Date Formula `fldY7kjaklLeoSgGd` - `Read-Only`
    
    Demonstrates all date-related Airtable functions using First Date, Second Date, and Third Date fields.
    
    ```
    "YEAR: " & YEAR({First Date})
    & ", MONTH: " & MONTH({First Date})
    & ", DAY: " & DAY({First Date})
    & ", HOUR: " & HOUR({Second Date})
    & ", MINUTE: " & MINUTE({Second Date})
    & ", SECOND: " & SECOND({Second Date})
    & ", TODAY: " & DATETIME_FORMAT(
      TODAY(),
      'YYYY-MM-DD'
    )
    & ", WORKDAY+5: " & DATETIME_FORMAT(
      WORKDAY({First Date}, 5),
      'YYYY-MM-DD'
    )
    & ", WORKDAY_DIFF: " & WORKDAY_DIFF({First Date}, {Second Date})
    … (truncated)
    ```
    """
    first_date: DateField = DateField('fldlZT521Iy0FFXFL')
    """First Date `fldlZT521Iy0FFXFL`"""
    first_number: NumberField = NumberField('fldA04pqfjMkGXcZU')
    """First Number `fldA04pqfjMkGXcZU`"""
    first_text: TextField = TextField('fldHJuw6pAujnHvkP')
    """First Text `fldHJuw6pAujnHvkP`"""
    math_formula: TextField = TextField('fldlSuvoeWGokSz8Z')
    """
    Math Formula `fldlSuvoeWGokSz8Z` - `Read-Only`
    
    Demonstrates all math-related functions using First Number and Second Number fields.
    
    ```
    IF(
      OR(
        {First Number} = BLANK(),
        {Second Number} = BLANK()
      ),
      BLANK(),
      "SUM: " & SUM({First Number}, {Second Number})
      & ", MIN: " & MIN({First Number}, {Second Number})
      & ", MAX: " & MAX({First Number}, {Second Number})
      & ", AVG: " & AVERAGE({First Number}, {Second Number})
      & ", COUNT: " & COUNT({First Number}, {Second Number})
      & ", CEIL: " & CEILING({First Number})
      & ", FLOOR: " & FLOOR({Second Number})
      & ", ROUND: " & ROUND({First Number}/2, 1)
      & ", ROUNDUP: " & ROUNDUP({Second Number}/2, 0)
    … (truncated)
    ```
    """
    primary_key: TextField = TextField('fldLZFrZKvSCS4dKb')
    """Primary Key `fldLZFrZKvSCS4dKb` - `Primary Key`"""
    second_date: DateField = DateField('fld1LZ3Ebpt0LaMmu')
    """Second Date `fld1LZ3Ebpt0LaMmu`"""
    second_number: NumberField = NumberField('fldj5nAkal5y8OOZg')
    """Second Number `fldj5nAkal5y8OOZg`"""
    second_text: TextField = TextField('fldA2boNwwsiuvXw1')
    """Second Text `fldA2boNwwsiuvXw1`"""
    text_formula: TextField = TextField('flddvzeqt7FJpQ9NX')
    """
    Text Formula `flddvzeqt7FJpQ9NX` - `Read-Only`
    
    Demonstrates all string-related Airtable functions using First Text, Second Text, and Third Text fields.
    
    ```
    CONCATENATE(
      "LEN: ",
      LEN({First Text}),
      "; ",
      "MID: ",
      MID({First Text}, 2, 3),
      "; ",
      "LEFT: ",
      LEFT({Second Text}, 2),
      "; ",
      "RIGHT: ",
      RIGHT({Second Text}, 2),
      "; ",
      "FIND: ",
      FIND("e", {First Text}),
    … (truncated)
    ```
    """
    third_date: DateField = DateField('fldxSQRRn8W879aiU')
    """Third Date `fldxSQRRn8W879aiU`"""
    third_number: NumberField = NumberField('fld5NBdekrAUzu4Fi')
    """Third Number `fld5NBdekrAUzu4Fi`"""
    third_text: TextField = TextField('fldfruPf8V9K6qIAN')
    """Third Text `fldfruPf8V9K6qIAN`"""

# endregion

