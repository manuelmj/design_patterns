import logging 
import pytest 
from creational.builder.builder import User, UserBuilder,Role

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def test_user_builder_complete():
    """Test para la construcción completa de un usuario"""
    user = (UserBuilder()
            .with_name("Alice")
            .with_email("alice@example.com")
            .with_age(30)
            .with_activo(True)
            .with_role(Role.ADMIN)
            .build())

    logger.info("Usuario creado: %s", user)
    assert user.name == "Alice"
    assert user.email == "alice@example.com"
    assert user.age == 30
    assert user.activo is True
    assert user.role == Role.ADMIN

def test_user_builder_minimal():
    """Test para la construcción mínima de un usuario"""
    user = (UserBuilder()
            .with_name("Bob")
            .with_email("bob@gmail.com")
            .build())
    
    logger.info("Usuario creado: %s", user)
    assert user.name == "Bob"
    assert user.email == "bob@gmail.com"
    assert user.age is None
    assert user.activo is True
    assert user.role == Role.GUEST



def test_user_builder_missing_name():
    """Test para la construcción de un usuario sin nombre"""
    with pytest.raises(ValueError, match="Name is required"):
        user = (UserBuilder()
                .with_email("charlie@example.com")
                .with_age(25)
                .with_activo(True)
                .with_role(Role.USER)
                .build())

def test_user_builder_missing_email():
    """Test para la construcción de un usuario sin email"""
    with pytest.raises(ValueError, match="Email is required"):
        user = (UserBuilder()
                .with_name("David")
                .with_age(40)
                .with_activo(False)
                .with_role(Role.USER)
                .build())

def test_user_builder_missing_name_and_email():
    """Test para la construcción de un usuario sin nombre y email"""
    with pytest.raises(ValueError, match="Name is required"):
        user = (UserBuilder()
                .with_age(22)
                .with_activo(True)
                .with_role(Role.GUEST)
                .build())