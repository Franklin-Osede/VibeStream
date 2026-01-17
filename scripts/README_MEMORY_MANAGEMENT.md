# 📦 Scripts de Gestión de Memoria - Vibestream

Scripts simplificados para gestionar la memoria del proyecto instalando y eliminando dependencias automáticamente.

## 🎯 Objetivo

Ahorrar RAM instalando dependencias solo cuando se necesitan y eliminándolas al terminar el desarrollo.

## 📋 Scripts Disponibles

### 🚀 `dev-start.sh`
Inicia el servidor de desarrollo instalando dependencias si no existen.

**Funcionalidades:**
- ✅ Verifica si `node_modules` existe
- ✅ Instala dependencias automáticamente si faltan
- ✅ Inicia el servidor con `npm run dev`
- ✅ Guarda el PID en `.pid` para gestión del proceso
- ✅ Verifica que el servidor inició correctamente

**Uso:**
```bash
./scripts/dev-start.sh
```

**Puerto:** 3000  
**Comando:** `npm run dev`

---

### 🛑 `dev-stop.sh`
Detiene el servidor y elimina `node_modules` para liberar RAM.

**Funcionalidades:**
- ✅ Detiene el servidor usando el PID guardado
- ✅ Elimina `node_modules` para ahorrar RAM
- ✅ Limpia el archivo `.pid`

**Uso:**
```bash
./scripts/dev-stop.sh
```

---

### 🧹 `cleanup.sh`
Limpieza manual sin detener el servidor.

**Funcionalidades:**
- ✅ Elimina `node_modules` sin afectar el servidor en ejecución
- ✅ Útil cuando necesitas liberar RAM pero el servidor sigue corriendo

**Uso:**
```bash
./scripts/cleanup.sh
```

**Nota:** Si el servidor está corriendo y eliminas `node_modules`, necesitarás ejecutar `npm install` antes de reiniciar.

---

## 🔧 Configuración

Los scripts están configurados para:
- **Ruta del proyecto:** `/Users/domoblock/Documents/Proycts-dev/Vibestream`
- **Puerto:** `3000`
- **Comando start:** `npm run dev`
- **Archivo PID:** `.pid` (en la raíz del proyecto)

## 📝 Permisos de Ejecución

Los scripts ya tienen permisos de ejecución. Si necesitas otorgarlos manualmente:

```bash
chmod +x scripts/dev-start.sh
chmod +x scripts/dev-stop.sh
chmod +x scripts/cleanup.sh
```

## 🔄 Flujo de Trabajo Recomendado

1. **Iniciar desarrollo:**
   ```bash
   ./scripts/dev-start.sh
   ```
   - Instala dependencias si no existen
   - Inicia el servidor en puerto 3000

2. **Desarrollar normalmente**

3. **Terminar sesión:**
   ```bash
   ./scripts/dev-stop.sh
   ```
   - Detiene el servidor
   - Elimina `node_modules` para liberar RAM

4. **Limpieza manual (opcional):**
   ```bash
   ./scripts/cleanup.sh
   ```
   - Útil si necesitas liberar RAM sin detener el servidor

## ⚠️ Notas Importantes

- El archivo `.pid` se guarda en la raíz del proyecto
- Si el servidor ya está corriendo, `dev-start.sh` no iniciará otro proceso
- Al eliminar `node_modules`, las dependencias se reinstalarán automáticamente en el próximo `dev-start.sh`
- El servidor se ejecuta en segundo plano (`&`)

## 🐛 Solución de Problemas

**El servidor no inicia:**
- Verifica que el puerto 3000 esté libre
- Revisa los logs del proceso
- Asegúrate de que `package.json` tenga el script `dev`

**El PID no se encuentra:**
- El archivo `.pid` puede haberse eliminado manualmente
- Verifica procesos con: `lsof -i :3000`

**Dependencias no se instalan:**
- Verifica conexión a internet
- Revisa que `package.json` y `package-lock.json` existan
