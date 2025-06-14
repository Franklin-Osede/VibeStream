const { getDefaultConfig } = require('expo/metro-config');

const config = getDefaultConfig(__dirname);

// Habilitamos Fast Refresh para mejor experiencia de desarrollo
config.server = {
  ...config.server,
  reloadOnChange: true,
  enhanceMiddleware: (middleware) => {
    return (req, res, next) => {
      // Agregamos headers para desarrollo
      res.setHeader('Access-Control-Allow-Origin', '*');
      return middleware(req, res, next);
    };
  },
};

// Optimizamos la resolución de módulos
config.resolver = {
  ...config.resolver,
  assetExts: [...config.resolver.assetExts, 'png', 'jpg', 'jpeg', 'gif', 'webp', 'svg'],
};

// Configuramos transformaciones para mejor performance
config.transformer = {
  ...config.transformer,
  minifierConfig: {
    mangle: {
      keep_fnames: true,
    },
  },
};

module.exports = config; 