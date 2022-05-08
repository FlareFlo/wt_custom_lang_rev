const Encore = require('@symfony/webpack-encore')
const { resolve } = require('path')

// Manually configure the runtime environment if not already configured yet by the "encore" command.
// It's useful when you use tools that rely on webpack.config.js file.
if (!Encore.isRuntimeEnvironmentConfigured()) {
	Encore.configureRuntimeEnvironment(process.env.NODE_ENV || 'dev')
}

/**
 * Encore is used instead of Webpack because it simply
 * is a wrapper around Webpack and makes it easier to use.
 *
 * It's a powerful shim over Webpack used in Symfony ecosystem,
 * having various power features (e.g. asset versioning).
 *
 * It is used in this project simply for it's easiness.
 */
Encore
	// directory where compiled assets will be stored
	.setOutputPath('dist/')
	// public path used by the web server to access the output path
	.setPublicPath('/')

	.addEntry('index', '/src/index.js')
	.addEntry('entries', '/src/entries.js')
	.addEntry('replace_header', '/src/replace_header.js')

	.copyFiles({
		from: './dev/assets',
		to: 'assets/[path][name].[ext]'
	})
	.copyFiles({
		from: './scss/out',
		to: 'css/[path][name].[ext]'
	})
	.copyFiles({
		from: './dev'
	})

	/**
	 * Having everything bundled in a single file is fine for this case.
	 *
	 * We don't care to load separate JS files since it's built right inside the final game exe.
	 * Additionally, it will be more complicated to load scripts during runtime, as being more like a Symfony thing.
	 *
	 * So let this as-is for now.
	 */
	.disableSingleRuntimeChunk()

	/*
	 * FEATURE CONFIG
	 *
	 * Enable & configure other features below. For a full
	 * list of features, see:
	 * https://symfony.com/doc/current/frontend.html#adding-more-features
	 */
	.cleanupOutputBeforeBuild()
	/**
	 * Let source maps be generated for dev environment.
	 * It will help you easier debug your game.
	 */
	.enableSourceMaps(!Encore.isProduction())

	/**
	 * This one enables @babel/preset-env polyfills
	 */
	.configureBabelPresetEnv((config) => {
		config.useBuiltIns = 'usage'
		config.corejs = 3
	})

	/**
	 * This one enables @webpack/typescript-loader
	 *
	 * I don't know how tsConfig override internally works, tho....
	 */
	.enableTypeScriptLoader(function (tsConfig) {
		tsConfig = require('./tsconfig.json')
	})

/**
 * This is how you get the raw webpack configuration to tweak it further beyond @symfony/encore's capabilities.
 * @type {webpack.Configuration}
 */
const config = Encore.getWebpackConfig()

/**
 * If you use TypeScript paths feature (to import @game/bla),
 * map your imports here to their directory source.
 *
 * Otherwise, WebPack will complain about modules not being existent.
 * @type {Record<string, string>}
 */
config.resolve.alias = {
	// '@game/': resolve(__dirname, 'src/')
}

/**
 * ...finally, we export the actual Webpack configuration.
 * @type {webpack.Configuration}
 */
module.exports = config