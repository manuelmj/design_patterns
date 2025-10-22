import logging 
import pytest 
from creational.Abstract_Factory_method.abstract_factory import (GetGUIFactory, OSType)

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def test_windows_factory():
    """Test para la fábrica de Windows"""
    factory = GetGUIFactory.get_factory(OSType.WINDOWS)
    button = factory.create_button()
    checkbox = factory.create_checkbox()

    assert button.paint() == "Rendering a button in Windows style"
    assert checkbox.paint() == "Rendering a checkbox in Windows style"

    logger.info(button.paint())
    logger.info(checkbox.paint())

def test_macos_factory():
    """Test para la fábrica de MacOS"""
    factory = GetGUIFactory.get_factory(OSType.MACOS)
    button = factory.create_button()
    checkbox = factory.create_checkbox()

    assert button.paint() == "Rendering a button in MacOS style"
    assert checkbox.paint() == "Rendering a checkbox in MacOS style"

    logger.info(button.paint())
    logger.info(checkbox.paint())

def test_unknown_os_type():
    """Test para tipo de OS desconocido"""
    with pytest.raises(ValueError, match="Unknown OS type"):
        GetGUIFactory.get_factory("unknown_os")