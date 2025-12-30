import hashlib


def test_ax1000000():
    h = hashlib.sha256()
    m = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'.encode()
    for _ in range(25000):
        h.update(m)
    assert h.hexdigest() == 'cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0'


def test_chain():
    m = bytearray(32)
    for _ in range(25000):
        h = hashlib.sha256()
        h.update(m)
        m = h.digest()
    assert m.hex() == 'de492c0e28d37706dac04897e9c825ee014b1e85bc277b3cddbd9a566451d7b6'


def test_empty():
    h = hashlib.sha256()
    assert h.hexdigest() == 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'


def test_hello_world():
    h = hashlib.sha256()
    h.update(b'Hello, World!')
    assert h.hexdigest() == 'dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f'


def test_len_57():
    h = hashlib.sha256()
    h.update(bytearray([1] * 57))
    assert h.hexdigest() == 'b6d6bcbe1aca250cc844a9c5633ab90c3f74f53ec3cda5950a8d9a4a77b086b4'


def test_rc4_16():
    h = hashlib.sha256()
    h.update(bytearray([
        0xde, 0x18, 0x89, 0x41, 0xa3, 0x37, 0x5d, 0x3a, 0x8a, 0x06, 0x1e, 0x67, 0x57, 0x6e, 0x92, 0x6d,
    ]))
    assert h.hexdigest() == '067c531269735ca7f541fdaca8f0dc76305d3cada140f89372a410fe5eff6e4d'


def test_rc4_55():
    h = hashlib.sha256()
    h.update(bytearray([
        0xde, 0x18, 0x89, 0x41, 0xa3, 0x37, 0x5d, 0x3a, 0x8a, 0x06, 0x1e, 0x67, 0x57, 0x6e, 0x92, 0x6d,
        0xc7, 0x1a, 0x7f, 0xa3, 0xf0, 0xcc, 0xeb, 0x97, 0x45, 0x2b, 0x4d, 0x32, 0x27, 0x96, 0x5f, 0x9e,
        0xa8, 0xcc, 0x75, 0x07, 0x6d, 0x9f, 0xb9, 0xc5, 0x41, 0x7a, 0xa5, 0xcb, 0x30, 0xfc, 0x22, 0x19,
        0x8b, 0x34, 0x98, 0x2d, 0xbb, 0x62, 0x9e,
    ]))
    assert h.hexdigest() == '038051e9c324393bd1ca1978dd0952c2aa3742ca4f1bd5cd4611cea83892d382'


test_ax1000000()
test_chain()
test_empty()
test_hello_world()
test_len_57()
test_rc4_16()
test_rc4_55()
