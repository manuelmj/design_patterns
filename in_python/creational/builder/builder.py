from abc import ABC, abstractmethod
from enum import Enum
from typing import Optional, Self


class Role(Enum): 
    """Enumeración de roles de usuario"""
    ADMIN = "Admin"
    GUEST = "Guest"
    USER = "User"


class User: 
    def __init__(self, name: str, email: str, age: Optional[int], activo: bool, role: Role):
        """Clase que representa un usuario"""
        self.name: str = name
        self.email: str = email
        self.age: Optional[int] = age
        self.activo: bool = activo
        self.role: Role = role
    
    def __str__(self) -> str:
        return (f"User(Name: {self.name}, Email: {self.email}, Age: {self.age}, "
                f"Activo: {self.activo}, Role: {self.role.value})")


class UserBuilder:

    def __init__(self):
        self._name: Optional[str] = None
        self._email: Optional[str] = None
        self._age : Optional[int] = None
        self._activo : bool = True
        self._role: Role = Role.GUEST


    def with_name(self,name:str) -> Self:
        """Setter para el nombre del usuario"""
        self._name = name
        return self

    def with_email(self,email: str) -> Self:
        """Setter para el email del usuario"""
        self._email = email
        return self

    def with_age(self,age:int) -> Self:
        """Setter para la edad del usuario"""
        self._age = age
        return self

    def with_activo(self,activo:bool) -> Self:
        """Setter para el estado activo del usuario"""
        self._activo = activo
        return self

    def with_role(self,role: Role) -> Self:
        """Setter para el rol del usuario"""
        self._role = role
        return self
    
    def build(self) -> User:
        """Construye el objeto User"""
        if not self._name:
            raise ValueError("Name is required")
        if not self._email:
            raise ValueError("Email is required")
       
        return User(
            name=self._name,
            email=self._email,
            age=self._age,
            activo=self._activo,
            role=self._role
        )
