# Phone Validator
A rust interface for python to check phone number

## Build wheel
```
git clone repo
cd phone_validator
python -m venv venv
source venv/bin/activate
pip install -U pip maturin
maturin build -r
```

## Install
```
pip install /path/to/phone_validator/target/wheels/*.whl
```

## Usage::
```py
>>> import phone_validator
>>> phone_validator.check_number('+8612345678901', '86', 11)
'12345678901'
>>> phone_validator.check_number('+8612345678901', '86', 9)
''
```
