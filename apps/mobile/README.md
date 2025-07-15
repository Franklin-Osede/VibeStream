# VibeStream Mobile App

## 🎵 **Plataforma de Streaming Musical con Funcionalidades Únicas**

VibeStream es una aplicación móvil revolucionaria que combina streaming de música tradicional con funcionalidades blockchain, VR y trading fraccional.

## ✨ **Funcionalidades Principales**

### 🎧 **Streaming de Música**
- Reproducción de alta calidad
- Recomendaciones personalizadas
- Playlists colaborativas
- Historial de reproducción

### 🥽 **Conciertos VR**
- Eventos en vivo en realidad virtual
- Capacidad limitada por evento
- Integración con Meta Quest y otros dispositivos VR
- Experiencias inmersivas únicas

### 🎨 **NFTs Musicales**
- Colecciones exclusivas de artistas
- Regalías automáticas para creadores
- Marketplace integrado
- Raridades y atributos únicos

### 📈 **Trading Fraccional**
- Compra/venta de acciones de canciones
- Portfolio personal de inversiones
- Análisis de rendimiento
- Mercado 24/7

### 💰 **Sistema de Vibers**
- Moneda interna para recompensas
- Ganar Vibers por escuchar música
- Gastar en eventos VR y NFTs
- Sistema de lealtad

## 🏗️ **Arquitectura Técnica**

### **Domain-Driven Design (DDD)**
```
src/
├── domain/           # Entidades y lógica de negocio
│   ├── entities/     # User, Song, etc.
│   └── repositories/ # Interfaces de acceso a datos
├── application/      # Casos de uso y servicios
│   ├── use-cases/    # GetHomeFeedUseCase, etc.
│   └── services/     # Servicios de aplicación
├── infrastructure/   # Implementaciones técnicas
│   ├── api/          # Cliente REST
│   └── services/     # BackendSyncService, etc.
├── presentation/     # Componentes React Native
│   ├── screens/      # Pantallas de la app
│   └── components/   # Componentes reutilizables
└── types/           # Tipos TypeScript centralizados
```

### **Tecnologías Utilizadas**
- **React Native** + **Expo** - Framework móvil
- **TypeScript** - Tipado estático
- **React Navigation** - Navegación
- **Expo Linear Gradient** - Efectos visuales
- **Jest** - Testing
- **ESLint** - Linting

## 🚀 **Instalación y Ejecución**

### **Prerrequisitos**
- Node.js 18+
- npm o yarn
- Expo CLI
- iOS Simulator o Android Emulator

### **Instalación**
```bash
# Clonar el repositorio
git clone https://github.com/vibestream/vibestream-mobile.git
cd vibestream-mobile

# Instalar dependencias
npm install

# Verificar tipos TypeScript
npm run type-check

# Ejecutar tests
npm test
```

### **Ejecución**
```bash
# Iniciar en modo desarrollo
npm start

# Ejecutar en iOS
npm run ios

# Ejecutar en Android
npm run android

# Ejecutar en web
npm run web
```

## 📱 **Estructura de Navegación**

### **5 Pestañas Principales**
1. **Your Feed** - Feed personalizado con posts y canciones
2. **Trending** - Contenido popular con filtros
3. **Explore** - Descubrimiento y búsqueda
4. **Library** - Biblioteca personal
5. **Notifications** - Actividad y notificaciones

### **Flujo de Usuario**
```
Login → Role Selection → Main App (Tab Navigator)
                    ↓
            Artist Dashboard / Fan Dashboard
```

## 🔗 **Integración con Backend**

### **OpenAPI 3.1.0**
- **15+ endpoints** RESTful
- **Autenticación JWT** Bearer tokens
- **WebSockets** para eventos en tiempo real
- **Event sourcing** para consistencia

### **Endpoints Principales**
- `POST /auth/login` - Inicio de sesión
- `GET /users/me` - Perfil del usuario
- `GET /songs` - Lista de canciones
- `POST /nfts` - Crear NFT
- `GET /royalties` - Pagos de regalías

### **Sincronización en Tiempo Real**
- **WebSocket** para eventos
- **Reconexión automática**
- **Cola de eventos pendientes**
- **Actualización optimista** del estado local

## 🧪 **Testing**

### **Cobertura de Tests**
- **Entidades de dominio** - User, Song
- **Casos de uso** - GetHomeFeedUseCase
- **Repositorios** - MockUserRepository, MockSongRepository
- **Hooks** - useHomeScreen

### **Ejecutar Tests**
```bash
# Tests unitarios
npm test

# Tests en modo watch
npm run test:watch

# Verificar cobertura
npm test -- --coverage
```

## 🔧 **Desarrollo**

### **Comandos Útiles**
```bash
# Verificar tipos TypeScript
npm run type-check

# Linting
npm run lint

# Linting con auto-fix
npm run lint:fix

# Build para producción
expo build:ios
expo build:android
```

### **Estructura de Datos Mock**
- **Users** - 3 usuarios de ejemplo
- **Songs** - 3 canciones con datos completos
- **VREvents** - 1 evento VR
- **NFTs** - 1 NFT de ejemplo
- **TradingPositions** - 1 posición de trading

## 📊 **Estado Actual**

### ✅ **Completado**
- Arquitectura DDD implementada
- Tipos TypeScript centralizados
- Navegación con 5 pestañas
- Mocks de datos completos
- Tests unitarios básicos
- Integración con backend preparada
- Funcionalidades únicas implementadas

### 🔄 **En Progreso**
- Corrección de errores de TypeScript
- Integración con backend real
- Implementación de WebRTC para VR
- Algoritmos de recomendación ML

### 📋 **Próximos Pasos**
1. **Conectar con Backend Real**
   - Implementar ApiClient
   - Configurar WebSocket
   - Manejar autenticación JWT

2. **Integrar Blockchain**
   - Conectar con contratos NFT
   - Implementar trading fraccional
   - Manejar transacciones ETH

3. **WebRTC para VR**
   - Streaming de audio/video
   - Gestión de salas virtuales
   - Sincronización de eventos

4. **ML para Recomendaciones**
   - Algoritmos personalizados
   - Análisis de comportamiento
   - Optimización de contenido

## 🤝 **Contribución**

### **Guidelines**
- Seguir principios DDD
- Escribir tests para nuevas funcionalidades
- Usar TypeScript estrictamente
- Documentar cambios importantes

### **Flujo de Trabajo**
1. Fork del repositorio
2. Crear rama feature: `git checkout -b feature/nueva-funcionalidad`
3. Commit cambios: `git commit -m 'feat: agregar nueva funcionalidad'`
4. Push a la rama: `git push origin feature/nueva-funcionalidad`
5. Crear Pull Request

## 📄 **Licencia**

Este proyecto está bajo la licencia MIT. Ver [LICENSE](LICENSE) para más detalles.

## 🆘 **Soporte**

Para soporte técnico o preguntas:
- 📧 Email: support@vibestream.com
- 💬 Discord: [VibeStream Community](https://discord.gg/vibestream)
- 📖 Documentación: [docs.vibestream.com](https://docs.vibestream.com)

---

**VibeStream** - El futuro de la música está aquí 🎵✨ 