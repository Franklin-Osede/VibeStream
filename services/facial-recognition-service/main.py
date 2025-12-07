#!/usr/bin/env python3
"""
Facial Recognition Service - Open Source Implementation
========================================================

Microservicio gratuito para reconocimiento facial usando face_recognition (dlib).

Uso:
    python main.py

Endpoints:
    POST /register - Registrar template facial
    POST /verify - Verificar coincidencia facial
    GET /health - Health check
"""

from flask import Flask, request, jsonify
from flask_cors import CORS
import face_recognition
import numpy as np
import base64
import io
import os
from PIL import Image
import json
import sqlite3
from datetime import datetime
import hashlib

app = Flask(__name__)
CORS(app)  # Permitir CORS para desarrollo

# Configuración
DB_PATH = os.getenv('DB_PATH', 'facial_templates.db')
SIMILARITY_THRESHOLD = float(os.getenv('SIMILARITY_THRESHOLD', '0.6'))

# Inicializar base de datos SQLite para almacenar templates
def init_db():
    """Inicializar base de datos para templates faciales"""
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()
    cursor.execute('''
        CREATE TABLE IF NOT EXISTS facial_templates (
            fan_id TEXT PRIMARY KEY,
            encoding BLOB NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    ''')
    conn.commit()
    conn.close()

init_db()

def get_db_connection():
    """Obtener conexión a la base de datos"""
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

@app.route('/health', methods=['GET'])
def health_check():
    """Health check endpoint"""
    return jsonify({
        'status': 'healthy',
        'service': 'facial-recognition-service',
        'version': '1.0.0'
    })

@app.route('/register', methods=['POST'])
def register_face():
    """
    Registrar template facial de un usuario
    
    Request:
        {
            "fan_id": "uuid-string",
            "image": "base64-encoded-image"
        }
    
    Response:
        {
            "success": true,
            "fan_id": "uuid-string",
            "message": "Face template registered successfully"
        }
    """
    try:
        data = request.json
        if not data:
            return jsonify({'error': 'No JSON data provided'}), 400
        
        fan_id = data.get('fan_id')
        image_base64 = data.get('image')
        
        if not fan_id:
            return jsonify({'error': 'fan_id is required'}), 400
        if not image_base64:
            return jsonify({'error': 'image is required'}), 400
        
        # Decodificar imagen
        try:
            image_bytes = base64.b64decode(image_base64)
            image = face_recognition.load_image_file(io.BytesIO(image_bytes))
        except Exception as e:
            return jsonify({'error': f'Invalid image format: {str(e)}'}), 400
        
        # Extraer encoding facial
        encodings = face_recognition.face_encodings(image)
        if not encodings:
            return jsonify({'error': 'No face detected in image'}), 400
        
        if len(encodings) > 1:
            return jsonify({'error': 'Multiple faces detected. Please provide image with single face'}), 400
        
        # Convertir encoding a bytes para almacenar
        encoding_bytes = encodings[0].tobytes()
        
        # Guardar en base de datos
        conn = get_db_connection()
        cursor = conn.cursor()
        cursor.execute('''
            INSERT OR REPLACE INTO facial_templates (fan_id, encoding, updated_at)
            VALUES (?, ?, ?)
        ''', (fan_id, encoding_bytes, datetime.utcnow().isoformat()))
        conn.commit()
        conn.close()
        
        return jsonify({
            'success': True,
            'fan_id': fan_id,
            'message': 'Face template registered successfully'
        }), 200
        
    except Exception as e:
        return jsonify({'error': f'Internal server error: {str(e)}'}), 500

@app.route('/verify', methods=['POST'])
def verify_face():
    """
    Verificar que una imagen coincide con template almacenado
    
    Request:
        {
            "fan_id": "uuid-string",
            "image": "base64-encoded-image"
        }
    
    Response:
        {
            "success": true,
            "fan_id": "uuid-string",
            "confidence_score": 0.95,
            "is_match": true,
            "distance": 0.12
        }
    """
    try:
        data = request.json
        if not data:
            return jsonify({'error': 'No JSON data provided'}), 400
        
        fan_id = data.get('fan_id')
        image_base64 = data.get('image')
        
        if not fan_id:
            return jsonify({'error': 'fan_id is required'}), 400
        if not image_base64:
            return jsonify({'error': 'image is required'}), 400
        
        # Obtener template almacenado
        conn = get_db_connection()
        cursor = conn.cursor()
        cursor.execute('SELECT encoding FROM facial_templates WHERE fan_id = ?', (fan_id,))
        row = cursor.fetchone()
        conn.close()
        
        if not row:
            return jsonify({'error': 'Face not registered for this fan_id'}), 404
        
        stored_encoding = np.frombuffer(row['encoding'], dtype=np.float64)
        
        # Decodificar imagen a verificar
        try:
            image_bytes = base64.b64decode(image_base64)
            image = face_recognition.load_image_file(io.BytesIO(image_bytes))
        except Exception as e:
            return jsonify({'error': f'Invalid image format: {str(e)}'}), 400
        
        # Extraer encoding de la imagen
        encodings = face_recognition.face_encodings(image)
        if not encodings:
            return jsonify({
                'success': True,
                'fan_id': fan_id,
                'confidence_score': 0.0,
                'is_match': False,
                'distance': 1.0,
                'message': 'No face detected in image'
            }), 200
        
        if len(encodings) > 1:
            return jsonify({'error': 'Multiple faces detected. Please provide image with single face'}), 400
        
        # Comparar encodings
        distance = face_recognition.face_distance([stored_encoding], encodings[0])[0]
        
        # Convertir distancia a confidence score (0.0 - 1.0)
        # Distancia menor = más similar
        # Threshold típico: 0.6 (puedes ajustar con SIMILARITY_THRESHOLD)
        is_match = distance < SIMILARITY_THRESHOLD
        confidence = max(0.0, min(1.0, 1.0 - (distance / SIMILARITY_THRESHOLD)))
        
        return jsonify({
            'success': True,
            'fan_id': fan_id,
            'confidence_score': float(confidence),
            'is_match': is_match,
            'distance': float(distance),
            'threshold': SIMILARITY_THRESHOLD
        }), 200
        
    except Exception as e:
        return jsonify({'error': f'Internal server error: {str(e)}'}), 500

@app.route('/delete/<fan_id>', methods=['DELETE'])
def delete_face(fan_id):
    """Eliminar template facial de un usuario"""
    try:
        conn = get_db_connection()
        cursor = conn.cursor()
        cursor.execute('DELETE FROM facial_templates WHERE fan_id = ?', (fan_id,))
        conn.commit()
        deleted = cursor.rowcount
        conn.close()
        
        if deleted == 0:
            return jsonify({'error': 'Face template not found'}), 404
        
        return jsonify({
            'success': True,
            'fan_id': fan_id,
            'message': 'Face template deleted successfully'
        }), 200
        
    except Exception as e:
        return jsonify({'error': f'Internal server error: {str(e)}'}), 500

if __name__ == '__main__':
    port = int(os.getenv('PORT', 8004))
    debug = os.getenv('DEBUG', 'false').lower() == 'true'
    
    print(f"🚀 Facial Recognition Service iniciado en puerto {port}")
    print(f"📊 Threshold de similitud: {SIMILARITY_THRESHOLD}")
    print(f"💾 Base de datos: {DB_PATH}")
    
    app.run(host='0.0.0.0', port=port, debug=debug)
