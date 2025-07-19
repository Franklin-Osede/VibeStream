const { getDefaultConfig } = require('expo/metro-config');
const path = require('path');

const config = getDefaultConfig(__dirname);

// Deshabilitar completamente el watcher
config.watchFolders = [];
config.resolver.nodeModulesPaths = [path.resolve(__dirname, 'node_modules')];

// Configuración del servidor sin watch
config.server = {
  port: 8081,
  enhanceMiddleware: (middleware) => {
    return (middleware);
  },
};

// Resolución de módulos muy restrictiva
config.resolver = {
  ...config.resolver,
  platforms: ['ios', 'android', 'native'],
  assetExts: [...config.resolver.assetExts, 'png', 'jpg', 'jpeg', 'gif', 'webp', 'svg'],
  // Bloquear TODOS los directorios externos
  blockList: [
    /.*\/\.\.\/.*/, // Excluir cualquier directorio padre
    /.*\/backend\/.*/,
    /.*\/services\/.*/,
    /.*\/contracts\/.*/,
    /.*\/circuits\/.*/,
    /.*\/docs\/.*/,
    /.*\/infra\/.*/,
    /.*\/migrations\/.*/,
    /.*\/scripts\/.*/,
    /.*\/shared\/.*/,
    /.*\/target\/.*/,
    /.*\/logs\/.*/,
    /.*\/dashboard\/.*/,
    /.*\/config\/.*/,
    /.*\/apps\/web\/.*/,
    /.*\/node_modules\/.*\/node_modules\/.*/,
  ],
};

// Configuración mínima de transformaciones
config.transformer = {
  ...config.transformer,
  minifierConfig: {
    mangle: {
      keep_fnames: true,
    },
  },
};

// Configuración para evitar problemas de archivos
config.maxWorkers = 1;
config.resetCache = true;

module.exports = config; 