module.exports = {
	output: {
		assetModuleFilename: "[path]/assets/[name][ext]"
	},
	module: {
		rules: [
			{
				test: /\.svg$/,
				type: "asset/resource"
			}
		]
	}
};
