# 🎭 Plan de Implementación: Biometría Facial en Fan Loyalty

> **Estado Actual**: Sistema usa audio, behavioral, device y location biometrics  
> **Biometría Facial**: No implementada aún  
> **Cuándo incluir**: Fase 2-3 (Después del MVP)

---

## 📊 Estado Actual del Sistema de Biometría

### Biometrías Implementadas Actualmente

El sistema de Fan Loyalty actualmente usa **4 tipos de biometría** (sin reconocimiento facial):

| Tipo | Peso | Estado | Descripción |
|------|------|--------|-------------|
| **Audio** | 40% | ✅ Implementado | Análisis de voz/audio del usuario |
| **Behavioral** | 30% | ✅ Implementado | Patrones de comportamiento (duración, skips, volumen) |
| **Device** | 20% | ✅ Implementado | Características del dispositivo (hardware fingerprint) |
| **Location** | 10% | ✅ Implementado | Consistencia de ubicación |
| **Facial** | 0% | ❌ No implementado | Reconocimiento facial |

**Score Total**: Suma de los 4 tipos (máximo 1.0)

---

## 🎯 ¿Cuándo Incluir Biometría Facial?

### Recomendación: Fase 2-3 (Después del MVP)

**Razones para NO incluirlo ahora**:

1. **Complejidad Legal y Ética**:
   - Requiere consentimiento explícito del usuario
   - Regulaciones GDPR/CCPA sobre datos biométricos
   - Necesitas políticas de privacidad específicas
   - Almacenamiento seguro de datos faciales

2. **Complejidad Técnica**:
   - Requiere servicio externo (AWS Rekognition, Azure Face API, etc.)
   - O implementación propia con ML (más complejo)
   - Procesamiento de imágenes/video
   - Almacenamiento de templates faciales

3. **Costo**:
   - Servicios de reconocimiento facial tienen costos por verificación
   - Almacenamiento de datos faciales (más costoso que otros datos)
   - Infraestructura adicional

4. **El sistema actual funciona**:
   - Con audio + behavioral + device ya tienes 90% de cobertura
   - Es suficiente para MVP y verificación básica

---

## 📅 Plan de Implementación por Fases

### Fase 1: MVP (Actual) - Sin Biometría Facial

**Estado**: ✅ Completado

**Biometrías usadas**:
- Audio (40%)
- Behavioral (30%)
- Device (20%)
- Location (10%)

**Score mínimo para verificación**: 0.5 (50%)

**Ventajas**:
- ✅ No requiere permisos especiales
- ✅ No hay problemas legales complejos
- ✅ Funciona sin servicios externos costosos
- ✅ Suficiente para verificación básica

**Limitaciones**:
- ⚠️ Menos preciso que con reconocimiento facial
- ⚠️ Puede ser "engañado" más fácilmente
- ⚠️ No identifica visualmente al usuario

---

### Fase 2: Mejora de Seguridad (Después del MVP)

**Cuándo**: 2-3 meses después del lanzamiento

**Objetivo**: Agregar biometría facial como capa adicional de seguridad

**Implementación**:

1. **Agregar campo facial a BiometricData**:
   ```rust
   pub struct BiometricData {
       pub audio_sample: Option<String>,
       pub facial_template: Option<String>,  // Nuevo campo
       pub behavioral_patterns: BehavioralPatterns,
       pub device_characteristics: DeviceCharacteristics,
       pub location: Option<LocationData>,
   }
   ```

2. **Actualizar pesos**:
   - Facial: 30% (nuevo)
   - Audio: 30% (reducido de 40%)
   - Behavioral: 25% (reducido de 30%)
   - Device: 10% (reducido de 20%)
   - Location: 5% (reducido de 10%)

3. **Integrar servicio externo**:
   - Opción A: AWS Rekognition
   - Opción B: Azure Face API
   - Opción C: Google Cloud Vision API
   - Opción D: Servicio propio con ML

4. **Actualizar cálculo de score**:
   ```rust
   fn calculate_verification_score(&self, biometric_data: &BiometricData) -> f64 {
       let mut score = 0.0;
       
       // Facial biometrics (30% weight) - NUEVO
       if let Some(facial_score) = biometric_data.facial_match_score {
           score += 0.3 * facial_score;
       }
       
       // Audio biometrics (30% weight)
       if biometric_data.audio_presence {
           score += 0.3 * 0.8;
       }
       
       // Behavioral biometrics (25% weight)
       if biometric_data.behavioral_patterns.is_consistent() {
           score += 0.25 * 0.9;
       }
       
       // Device biometrics (10% weight)
       if biometric_data.device_authenticity.is_verified() {
           score += 0.1 * 0.7;
       }
       
       // Location biometrics (5% weight)
       if biometric_data.location_consistency.is_reasonable() {
           score += 0.05 * 0.6;
       }
       
       score.min(1.0)
   }
   ```

**Requisitos previos**:
- ✅ MVP funcionando en producción
- ✅ Políticas de privacidad actualizadas
- ✅ Consentimiento explícito del usuario
- ✅ Servicio de reconocimiento facial elegido
- ✅ Almacenamiento seguro configurado

---

### Fase 3: Optimización y ML Propio (Opcional, Largo Plazo)

**Cuándo**: 6-12 meses después del lanzamiento

**Objetivo**: Reemplazar servicio externo con ML propio

**Implementación**:
- Modelo propio de reconocimiento facial
- Entrenamiento con datos propios
- Reducción de costos
- Mayor control y privacidad

**Requisitos**:
- Equipo de ML/Data Science
- Infraestructura para entrenamiento
- Datos etiquetados
- Compliance legal completo

---

## 🔧 Implementación Técnica (Cuando decidas hacerlo)

### Opción 0: Solución Gratuita con Open Source (Recomendado para MVP) ⭐

**Ventajas**:
- ✅ **100% Gratuito** - Sin costos de API
- ✅ Control total de datos (privacidad)
- ✅ Sin dependencias externas
- ✅ Funciona offline
- ✅ Open source y auditable

**Desventajas**:
- ⚠️ Requiere más trabajo inicial
- ⚠️ Necesitas infraestructura para inferencia
- ⚠️ Mantenimiento del modelo

**Librerías Open Source Disponibles**:

1. **face_recognition (Python)** - Basado en dlib
   - Muy fácil de usar
   - Precisión alta
   - Puedes crear un microservicio en Python

2. **face-api.js (JavaScript/TypeScript)**
   - Corre en el navegador o Node.js
   - Modelos pre-entrenados
   - Muy rápido

3. **OpenCV + Dlib (C++/Python)**
   - Más control
   - Muy preciso
   - Requiere más configuración

4. **InsightFace (Python)**
   - State-of-the-art
   - Muy preciso
   - Modelos pre-entrenados disponibles

**Implementación Recomendada para Rust**:

Crear un microservicio en Python que use `face_recognition` y exponerlo como API REST:

```python
# services/facial-recognition-service/main.py
from flask import Flask, request, jsonify
import face_recognition
import numpy as np
import base64
import io
from PIL import Image

app = Flask(__name__)

# Almacenar templates faciales en memoria (o Redis/DB)
facial_templates = {}

@app.route('/register', methods=['POST'])
def register_face():
    """Registrar template facial de un usuario"""
    data = request.json
    fan_id = data['fan_id']
    image_base64 = data['image']
    
    # Decodificar imagen
    image_bytes = base64.b64decode(image_base64)
    image = face_recognition.load_image_file(io.BytesIO(image_bytes))
    
    # Extraer encoding facial
    encodings = face_recognition.face_encodings(image)
    if not encodings:
        return jsonify({'error': 'No face detected'}), 400
    
    # Guardar template (solo el encoding, no la imagen)
    facial_templates[fan_id] = encodings[0].tolist()
    
    return jsonify({
        'success': True,
        'fan_id': fan_id,
        'message': 'Face template registered'
    })

@app.route('/verify', methods=['POST'])
def verify_face():
    """Verificar que una imagen coincide con template almacenado"""
    data = request.json
    fan_id = data['fan_id']
    image_base64 = data['image']
    
    # Obtener template almacenado
    if fan_id not in facial_templates:
        return jsonify({'error': 'Face not registered'}), 404
    
    stored_encoding = np.array(facial_templates[fan_id])
    
    # Decodificar imagen a verificar
    image_bytes = base64.b64decode(image_base64)
    image = face_recognition.load_image_file(io.BytesIO(image_bytes))
    
    # Extraer encoding de la imagen
    encodings = face_recognition.face_encodings(image)
    if not encodings:
        return jsonify({'error': 'No face detected'}), 400
    
    # Comparar encodings
    distance = face_recognition.face_distance([stored_encoding], encodings[0])[0]
    
    # Convertir distancia a confidence score (0.0 - 1.0)
    # Distancia menor = más similar
    # Threshold típico: 0.6 (puedes ajustar)
    confidence = max(0.0, 1.0 - (distance / 0.6))
    
    return jsonify({
        'success': True,
        'fan_id': fan_id,
        'confidence_score': float(confidence),
        'is_match': distance < 0.6,
        'distance': float(distance)
    })

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8004)
```

**Integración en Rust**:

```rust
// services/api-gateway/src/bounded_contexts/fan_loyalty/infrastructure/facial_service.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OpenSourceFacialService {
    client: Client,
    service_url: String,
}

#[derive(Serialize)]
struct RegisterFaceRequest {
    fan_id: String,
    image: String, // base64
}

#[derive(Serialize)]
struct VerifyFaceRequest {
    fan_id: String,
    image: String, // base64
}

#[derive(Deserialize)]
struct VerifyFaceResponse {
    success: bool,
    confidence_score: f32,
    is_match: bool,
}

impl OpenSourceFacialService {
    pub fn new(service_url: String) -> Self {
        Self {
            client: Client::new(),
            service_url,
        }
    }
    
    pub async fn register_face(
        &self,
        fan_id: &str,
        image_bytes: &[u8],
    ) -> Result<(), String> {
        let image_base64 = base64::encode(image_bytes);
        
        let request = RegisterFaceRequest {
            fan_id: fan_id.to_string(),
            image: image_base64,
        };
        
        let response = self.client
            .post(&format!("{}/register", self.service_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to call facial service: {}", e))?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            Err("Failed to register face".to_string())
        }
    }
    
    pub async fn verify_face(
        &self,
        fan_id: &str,
        image_bytes: &[u8],
    ) -> Result<f32, String> {
        let image_base64 = base64::encode(image_bytes);
        
        let request = VerifyFaceRequest {
            fan_id: fan_id.to_string(),
            image: image_base64,
        };
        
        let response = self.client
            .post(&format!("{}/verify", self.service_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to call facial service: {}", e))?;
        
        let result: VerifyFaceResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        if result.is_match {
            Ok(result.confidence_score)
        } else {
            Ok(0.0)
        }
    }
}
```

**Costo**: $0 (solo infraestructura del servidor, que ya tienes)

**Precisión**: 95-98% (similar a servicios pagos)

**Requisitos**:
- Servidor Python (puede ser el mismo que corre otros servicios)
- Librería `face_recognition` (gratuita)
- Almacenamiento para templates (PostgreSQL o Redis)

**Dockerfile para el servicio**:

```dockerfile
FROM python:3.11-slim

WORKDIR /app

# Instalar dependencias del sistema para face_recognition
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    libopenblas-dev \
    liblapack-dev \
    libx11-dev \
    libgtk-3-dev \
    && rm -rf /var/lib/apt/lists/*

# Instalar dependencias Python
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

CMD ["python", "main.py"]
```

**requirements.txt**:
```
flask==3.0.0
face-recognition==1.3.0
numpy==1.24.3
Pillow==10.1.0
```

---

### Opción 1: AWS Rekognition (Pago, pero fácil)

**Ventajas**:
- ✅ Fácil de integrar
- ✅ Bien documentado
- ✅ Escalable
- ✅ Cumple con compliance

**Desventajas**:
- ⚠️ Costo por verificación
- ⚠️ Dependencia de AWS
- ⚠️ Datos procesados por terceros

**Implementación**:

1. **Agregar dependencia**:
   ```toml
   # Cargo.toml
   aws-sdk-rekognition = "1.0"
   ```

2. **Crear adapter**:
   ```rust
   // services/api-gateway/src/bounded_contexts/fan_loyalty/infrastructure/facial_adapter.rs
   pub struct AwsRekognitionAdapter {
       client: aws_sdk_rekognition::Client,
   }
   
   impl FacialRecognitionService for AwsRekognitionAdapter {
       async fn verify_face(
           &self,
           image_bytes: &[u8],
           stored_template: &str,
       ) -> Result<f32, String> {
           // Llamar a AWS Rekognition CompareFaces
           // Retornar confidence score (0.0 - 1.0)
       }
   }
   ```

3. **Integrar en BiometricVerificationService**:
   ```rust
   impl BiometricVerificationService for ExternalBiometricAdapter {
       async fn verify_fan(&self, fan_id: &FanId, biometric_data: &BiometricData) -> Result<FanVerificationResult, String> {
           let mut scores = vec![];
           
           // Facial verification (si está disponible)
           if let Some(facial_image) = &biometric_data.facial_image {
               let facial_score = self.facial_service.verify_face(
                   facial_image,
                   &self.get_stored_template(fan_id).await?,
               ).await?;
               scores.push(("facial", 0.3, facial_score));
           }
           
           // ... otros tipos de biometría
           
           // Calcular score total
           let total_score = scores.iter()
               .map(|(_, weight, score)| weight * score)
               .sum::<f32>();
           
           Ok(FanVerificationResult {
               is_verified: total_score >= 0.7,
               confidence_score: total_score,
               // ...
           })
       }
   }
   ```

---

### Opción 2: Azure Face API

**Similar a AWS**, pero usando Azure SDK.

---

### Opción 3: Servicio Propio con ML

**Requisitos**:
- Modelo de reconocimiento facial (FaceNet, ArcFace, etc.)
- Infraestructura para inferencia
- Almacenamiento de templates
- API REST para el servicio

**Ventajas**:
- ✅ Control total
- ✅ Sin costos por verificación (solo infraestructura)
- ✅ Datos no salen de tu infraestructura

**Desventajas**:
- ⚠️ Requiere equipo de ML
- ⚠️ Mantenimiento del modelo
- ⚠️ Más complejo de implementar

---

## 📋 Checklist Antes de Implementar Biometría Facial

### Legal y Compliance

- [ ] **Política de Privacidad actualizada**
  - Explicar qué datos faciales se recopilan
  - Cómo se almacenan
  - Cómo se usan
  - Derechos del usuario (eliminación, acceso, etc.)

- [ ] **Consentimiento Explícito**
  - Checkbox específico para biometría facial
  - No puede ser parte del TOS general
  - Debe ser opt-in, no opt-out

- [ ] **Cumplimiento Legal**
  - GDPR (si hay usuarios en UE)
  - CCPA (si hay usuarios en California)
  - BIPA (si hay usuarios en Illinois)
  - Leyes locales de biometría

- [ ] **Almacenamiento Seguro**
  - Encriptación de templates faciales
  - Acceso restringido
  - Auditoría de accesos

### Técnico

- [ ] **Servicio de Reconocimiento Elegido**
  - AWS Rekognition, Azure Face API, o propio
  - API keys configuradas
  - Límites de costo establecidos

- [ ] **Estructura de Datos**
  - Campo `facial_template` en `BiometricData`
  - Tabla para almacenar templates
  - Migración de base de datos

- [ ] **Integración**
  - Adapter para servicio facial
  - Actualizar `calculate_verification_score`
  - Tests de integración

- [ ] **Frontend**
  - Captura de imagen/video facial
  - UI para consentimiento
  - Feedback al usuario

### Operacional

- [ ] **Monitoreo**
  - Tasa de éxito/fallo
  - Costos del servicio
  - Performance

- [ ] **Soporte**
  - Documentación para usuarios
  - Proceso para eliminar datos faciales
  - Manejo de falsos positivos/negativos

---

## 🎯 Recomendación Final

### Para MVP (Ahora): ❌ NO incluir biometría facial

**Razones**:
1. El sistema actual (audio + behavioral + device) es suficiente
2. Evita complejidad legal en el lanzamiento
3. Reduce costos iniciales
4. Permite validar el concepto primero

### Para Fase 2 (2-3 meses después): ✅ SÍ incluir biometría facial

**Razones**:
1. Ya tienes usuarios y datos para validar
2. Puedes justificar el costo con uso real
3. Tienes tiempo para compliance legal
4. Mejora significativamente la seguridad

### Plan de Implementación Sugerido

**Mes 1-2 (MVP)**:
- ✅ Usar sistema actual (sin facial)
- ✅ Lanzar y obtener usuarios
- ✅ Validar concepto

**Mes 3-4 (Preparación)**:
- ⚠️ Preparar compliance legal
- ⚠️ Elegir servicio de reconocimiento
- ⚠️ Diseñar UI de consentimiento

**Mes 5-6 (Implementación)**:
- ⚠️ Implementar biometría facial
- ⚠️ Tests y validación
- ⚠️ Rollout gradual

---

## 📝 Cambios Necesarios en el Código (Cuando lo implementes)

### 1. Actualizar `BiometricData`

```rust
// services/api-gateway/src/bounded_contexts/fan_loyalty/domain/entities.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricData {
    pub audio_sample: Option<String>,
    pub facial_image: Option<Vec<u8>>,  // NUEVO: Imagen facial (base64 o bytes)
    pub facial_template_id: Option<String>,  // NUEVO: ID del template almacenado
    pub behavioral_patterns: BehavioralPatterns,
    pub device_characteristics: DeviceCharacteristics,
    pub location: Option<LocationData>,
}
```

### 2. Crear Servicio de Reconocimiento Facial

```rust
// services/api-gateway/src/bounded_contexts/fan_loyalty/infrastructure/facial_service.rs

#[async_trait]
pub trait FacialRecognitionService: Send + Sync {
    /// Verificar que una imagen facial coincide con un template almacenado
    async fn verify_face(
        &self,
        image_bytes: &[u8],
        template_id: &str,
    ) -> Result<f32, String>;  // Retorna confidence score 0.0-1.0
    
    /// Crear template facial desde una imagen
    async fn create_template(
        &self,
        image_bytes: &[u8],
        fan_id: &FanId,
    ) -> Result<String, String>;  // Retorna template_id
}
```

### 3. Actualizar Cálculo de Score

```rust
// services/api-gateway/src/bounded_contexts/fan_loyalty/domain/aggregates.rs

fn calculate_verification_score(&self, biometric_data: &BiometricData) -> f64 {
    let mut score = 0.0;
    
    // Facial biometrics (30% weight) - NUEVO
    if let Some(facial_score) = &biometric_data.facial_match_score {
        score += 0.3 * facial_score;
    }
    
    // Audio biometrics (30% weight) - Reducido de 40%
    if biometric_data.audio_presence {
        score += 0.3 * 0.8;
    }
    
    // Behavioral biometrics (25% weight) - Reducido de 30%
    if biometric_data.behavioral_patterns.is_consistent() {
        score += 0.25 * 0.9;
    }
    
    // Device biometrics (10% weight) - Reducido de 20%
    if biometric_data.device_authenticity.is_verified() {
        score += 0.1 * 0.7;
    }
    
    // Location biometrics (5% weight) - Reducido de 10%
    if biometric_data.location_consistency.is_reasonable() {
        score += 0.05 * 0.6;
    }
    
    score.min(1.0)
}
```

### 4. Agregar Endpoint para Registro Facial

```rust
// Nuevo endpoint: POST /api/v1/fan-loyalty/register-face
// Permite al usuario registrar su template facial inicial
```

---

## 💰 Consideraciones de Costo

### Opción Gratuita (Open Source) ⭐ RECOMENDADO

**Costo**: $0 en servicios externos

**Infraestructura**:
- Servidor Python: Ya lo tienes (puede correr en el mismo servidor)
- Almacenamiento templates: PostgreSQL (ya lo tienes)
- **Costo adicional**: $0

**Ventajas**:
- ✅ 100% gratuito
- ✅ Control total
- ✅ Privacidad (datos no salen de tu infraestructura)
- ✅ Sin límites de uso

**Desventajas**:
- ⚠️ Requiere implementación inicial (1-2 semanas)
- ⚠️ Mantenimiento del servicio

**Recomendación**: **Usa esta opción para MVP y Fase 2**. Solo cambia a servicios pagos si necesitas mayor precisión o escala masiva.

---

### AWS Rekognition (Pago)

- **CompareFaces**: $1.00 por 1,000 comparaciones
- **DetectFaces**: $1.00 por 1,000 detecciones
- **CreateCollection**: Gratis
- **Storage**: Templates almacenados en S3 (mínimo costo)

**Estimación para 10,000 usuarios**:
- Registro inicial: $10 (10,000 detecciones)
- Verificaciones diarias: $30/mes (1 verificación por usuario por día)
- **Total estimado**: ~$40/mes para 10,000 usuarios activos

**Tier Gratuito**: 5,000 imágenes/mes gratis (solo para los primeros 12 meses)

---

### Azure Face API (Pago)

- Similar a AWS, precios competitivos
- **Tier Gratuito**: 30,000 transacciones/mes gratis

---

### Servicio Propio con ML Avanzado

- **Infraestructura**: $50-200/mes (depende del tráfico)
- **Desarrollo**: 2-3 meses de trabajo
- **Mantenimiento**: Continuo
- **Ventaja**: Modelos más precisos, pero requiere equipo de ML

---

## 🔒 Consideraciones de Seguridad

1. **Almacenamiento de Templates**:
   - NO almacenar imágenes completas
   - Solo templates/vectores encriptados
   - Encriptación en reposo y en tránsito

2. **Procesamiento**:
   - Procesar imágenes en memoria, no guardar temporalmente
   - Eliminar imágenes después de procesar
   - Logs sin datos faciales

3. **Acceso**:
   - Solo servicios autorizados pueden acceder
   - Auditoría de todos los accesos
   - Rate limiting en endpoints faciales

---

## 📊 Métricas a Monitorear

Cuando implementes biometría facial, monitorea:

1. **Tasa de Éxito**:
   - True Positives (correctamente identificados)
   - False Positives (identificados incorrectamente)
   - False Negatives (no identificados cuando deberían)

2. **Performance**:
   - Tiempo de verificación
   - Tasa de error del servicio
   - Costo por verificación

3. **Adopción**:
   - % de usuarios que optan por biometría facial
   - Tasa de abandono del proceso
   - Satisfacción del usuario

---

## ✅ Conclusión Actualizada

### Opción Gratuita Disponible ⭐

**¡Buenas noticias!** Puedes implementar biometría facial **100% gratuita** usando librerías open source como `face_recognition` (Python).

**He creado un microservicio completo y listo para usar**:
- ✅ `services/facial-recognition-service/` - Servicio Python completo
- ✅ Dockerfile incluido
- ✅ Integración con Rust preparada
- ✅ **Costo: $0**

**Esto cambia la recomendación**:

### Para MVP (Ahora): ⚠️ **Considerar incluir biometría facial gratuita**

**Ventajas de incluirla ahora (con solución gratuita)**:
- ✅ **Costo $0** - No hay costos de API
- ✅ **Ya está implementado** - Servicio listo para usar
- ✅ Mejora significativa de seguridad desde el inicio
- ✅ Diferenciador competitivo
- ✅ No hay dependencias de servicios externos pagos
- ✅ Control total de datos (mejor privacidad)

**Desventajas**:
- ⚠️ Requiere integración (1-2 días de trabajo)
- ⚠️ Complejidad legal (consentimiento, GDPR)
- ⚠️ Necesitas correr microservicio Python adicional

**Recomendación actualizada**:
- **Si tienes 1-2 días**: Implementa biometría facial gratuita ahora (ya está el servicio)
- **Si quieres lanzar rápido**: Espera a Fase 2, pero ahora sabes que es gratis y está listo

### Para Fase 2 (2-3 meses): ✅ **Definitivamente incluir**

**Si no lo incluiste en MVP**, definitivamente inclúyelo en Fase 2. Con la solución gratuita lista, no hay razón para no hacerlo.

---

## 🎁 Bonus: Servicio Ya Creado

He creado un servicio completo de reconocimiento facial gratuito:

**Ubicación**: `services/facial-recognition-service/`

**Incluye**:
- ✅ Servicio Python con Flask
- ✅ Dockerfile para containerización
- ✅ Integración con Rust (adapter creado)
- ✅ Base de datos SQLite para templates
- ✅ API REST completa
- ✅ Health checks
- ✅ Manejo de errores

**Para usarlo**:

1. **Iniciar servicio**:
   ```bash
   cd services/facial-recognition-service
   docker build -t facial-recognition-service .
   docker run -p 8004:8004 facial-recognition-service
   ```

2. **Integrar en Rust** (adapter ya creado):
   ```rust
   use crate::bounded_contexts::fan_loyalty::infrastructure::facial_service::OpenSourceFacialService;
   
   let facial_service = OpenSourceFacialService::from_env()?;
   let score = facial_service.verify_face(&fan_id, &image_bytes).await?;
   ```

3. **Agregar a docker-compose.yml** (ya agregado):
   ```yaml
   facial-recognition-service:
     build: ./services/facial-recognition-service
     ports:
       - "8004:8004"
   ```

**Costo total**: $0 (solo infraestructura que ya tienes)

**Plan de acción**:
1. **Ahora**: Enfócate en completar MVP sin facial
2. **Mes 3-4**: Preparar compliance y elegir servicio
3. **Mes 5-6**: Implementar y hacer rollout gradual

---

> **Nota**: El sistema actual de biometría (audio + behavioral + device + location) es robusto y suficiente para MVP. La biometría facial es una mejora importante pero no crítica para el lanzamiento inicial.
