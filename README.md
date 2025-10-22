# 🎯 Design Patterns

![Python](https://img.shields.io/badge/Python-3.8+-blue.svg)
![Rust](https://img.shields.io/badge/Rust-2021+-orange.svg)
![Status](https://img.shields.io/badge/Status-Work%20in%20Progress-yellow.svg)

Un repositorio completo de implementaciones de patrones de diseño en **Python** y **Rust**, con ejemplos prácticos, tests unitarios y documentación detallada.

## 📖 Descripción

Este proyecto tiene como objetivo proporcionar implementaciones claras y bien documentadas de los patrones de diseño más importantes en programación. Cada patrón está implementado en dos lenguajes diferentes para mostrar cómo los conceptos se adaptan a diferentes paradigmas de programación.

### 🚧 Estado del Proyecto

> **Nota:** Este repositorio está en desarrollo activo y se estará actualizando constantemente con nuevos patrones de diseño, mejoras en las implementaciones existentes y documentación adicional. Las contribuciones y sugerencias son bienvenidas.

## 🏗️ Estructura del Proyecto

```
design_patterns/
├── in_python/          # Implementaciones en Python
└── in_rust/           # Implementaciones en Rust
```

## 🎨 Patrones Implementados

### Patrones Creacionales

| Patrón | Python | Rust | Descripción |
|--------|--------|------|-------------|
| **Abstract Factory** | ✅ | ✅ | Proporciona una interfaz para crear familias de objetos relacionados |
| **Builder** | ✅ | ✅ | Construye objetos complejos paso a paso |
| **Factory Method** | ✅ | ✅ | Crea objetos sin especificar sus clases exactas |
| **Singleton** | 🔄 | 🔄 | Garantiza que una clase tenga una sola instancia |
| **Prototype** | 📅 | 📅 | Crea objetos clonando instancias existentes |

### Patrones Estructurales (Próximamente)

| Patrón | Python | Rust | Descripción |
|--------|--------|------|-------------|
| **Adapter** | 📅 | 📅 | Permite que interfaces incompatibles trabajen juntas |
| **Bridge** | 📅 | 📅 | Separa una abstracción de su implementación |
| **Composite** | 📅 | 📅 | Compone objetos en estructuras de árbol |
| **Decorator** | 📅 | 📅 | Añade funcionalidad a objetos dinámicamente |
| **Facade** | 📅 | 📅 | Proporciona una interfaz simplificada |
| **Flyweight** | 📅 | 📅 | Minimiza el uso de memoria compartiendo datos |
| **Proxy** | 📅 | 📅 | Proporciona un representante de otro objeto |

### Patrones Comportamentales (Próximamente)

| Patrón | Python | Rust | Descripción |
|--------|--------|------|-------------|
| **Observer** | 📅 | 📅 | Define dependencias uno-a-muchos entre objetos |
| **Strategy** | 📅 | 📅 | Encapsula algoritmos intercambiables |
| **Command** | 📅 | 📅 | Encapsula solicitudes como objetos |
| **State** | 📅 | 📅 | Permite cambiar el comportamiento según el estado |
| **Template Method** | 📅 | 📅 | Define el esqueleto de un algoritmo |

**Leyenda:**
- ✅ Implementado
- 🔄 En desarrollo
- 📅 Planificado

## 🚀 Inicio Rápido


### Instalación y Configuración

#### Python
```bash
# Clonar el repositorio
git clone https://github.com/manuelmj/design_patterns.git
cd design_patterns/in_python

nota: Asegurese de crear y activar un entorno virtual si es necesario.
# Instalar dependencias
pip install -r requirements.txt

# Ejecutar tests
make test

#ejecutar un test usando make 
make test-builder-method


# Ejecutar un ejemplo específico
pytest -v -s creational/Factory/facotry_test.py 
```

#### Rust
```bash
# Navegar al directorio de Rust
cd design_patterns/in_rust/creational

# Compilar el proyecto
cargo build

# Ejecutar tests
make test
#ejecutar un test específico usando make 
make test-builder-method 

# o alternativamente:
cargo test

# Ejecutar ejemplos
cargo run
```

## 🧪 Tests

Cada implementación incluye tests unitarios completos para garantizar la correcta funcionalidad de los patrones.

### Ejecutar Tests en Python
```bash
cd in_python
pytest
# o usar el Makefile
make test

# puede ejecutar tets especificos usando make y buscando el test correspondiente que desea ejecutar
```

### Ejecutar Tests en Rust
```bash
cd in_rust/creational
cargo test
# o usar el Makefile
make test

# puede ejecutar tets especificos usando make y buscando el test correspondiente que desea ejecutar
```


## 📖 Documentación

Cada patrón incluye:

- **Implementación completa** con comentarios explicativos
- **Tests unitarios** que demuestran el uso correcto
- **Ejemplos prácticos** de casos de uso reales
- **Comparación** entre implementaciones en Python y Rust

### Recursos Adicionales

- [Patrones de Diseño - Refactoring Guru](https://refactoring.guru/design-patterns)

