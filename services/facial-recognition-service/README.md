# Facial Recognition Service - Open Source

Microservicio gratuito para reconocimiento facial usando `face_recognition` (basado en dlib).

## 🎯 Características

- ✅ **100% Gratuito** - Sin costos de API externos
- ✅ **Open Source** - Basado en face_recognition (dlib)
- ✅ **Precisión Alta** - 95-98% de precisión
- ✅ **Privacidad** - Datos no salen de tu infraestructura
- ✅ **Sin Límites** - Sin restricciones de uso

## 🚀 Inicio Rápido

### Con Docker (Recomendado)

```bash
cd services/facial-recognition-service
docker build -t facial-recognition-service .
docker run -p 8004:8004 facial-recognition-service
```

### Sin Docker

```bash
# Instalar dependencias del sistema (Ubuntu/Debian)
sudo apt-get install -y build-essential cmake libopenblas-dev liblapack-dev libx11-dev libgtk-3-dev

# Instalar dependencias Python
pip install -r requirements.txt

# Ejecutar
python main.py
```

## 📡 API Endpoints

### POST /register

Registrar template facial de un usuario.

**Request**:
```json
{
  "fan_id": "550e8400-e29b-41d4-a716-446655440000",
  "image": "base64-encoded-image"
}
```

**Response**:
```json
{
  "success": true,
  "fan_id": "550e8400-e29b-41d4-a716-446655440000",
  "message": "Face template registered successfully"
}
```

### POST /verify

Verificar que una imagen coincide con template almacenado.

**Request**:
```json
{
  "fan_id": "550e8400-e29b-41d4-a716-446655440000",
  "image": "base64-encoded-image"
}
```

**Response**:
```json
{
  "success": true,
  "fan_id": "550e8400-e29b-41d4-a716-446655440000",
  "confidence_score": 0.95,
  "is_match": true,
  "distance": 0.12,
  "threshold": 0.6
}
```

### DELETE /delete/:fan_id

Eliminar template facial de un usuario.

### GET /health

Health check.

## ⚙️ Configuración

Variables de entorno:

- `PORT`: Puerto del servicio (default: 8004)
- `DEBUG`: Modo debug (default: false)
- `SIMILARITY_THRESHOLD`: Threshold para considerar match (default: 0.6)
- `DB_PATH`: Ruta a base de datos SQLite (default: facial_templates.db)

## 🔧 Integración con Rust

Ver `BIOMETRIA_FACIAL_PLAN.md` para código de integración en Rust.

## 📊 Performance

- **Tiempo de registro**: ~200-500ms
- **Tiempo de verificación**: ~100-300ms
- **Precisión**: 95-98%
- **Falsos positivos**: < 1% (con threshold 0.6)

## 🔒 Seguridad

- Templates faciales almacenados encriptados (opcional)
- Solo almacena encodings, no imágenes completas
- Base de datos SQLite local (puede migrarse a PostgreSQL)

## 💡 Notas

- Requiere imágenes con una sola cara
- Funciona mejor con imágenes frontales
- Iluminación adecuada mejora precisión
- Threshold ajustable según necesidades de seguridad
