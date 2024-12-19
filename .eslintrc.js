const path = require("path");

module.exports = exports = {
    "parser": "@typescript-eslint/parser",
    "plugins": [
        "@typescript-eslint"
    ],
    "extends": [
        "eslint:recommended",
        "plugin:@typescript-eslint/recommended",
    ],
    "parserOptions": {
         "project": path.join(__dirname, "tsconfig.json"),
    },
    "rules": {
        "max-len": [
            "error",
            {
                "code": 100
            }
        ],
        "quotes": [
            "error",
            "double",
            {
                "avoidEscape": true,
                "allowTemplateLiterals": true,
            }
        ],
        "@typescript-eslint/no-unused-vars": [
            "warn",
            {
                "argsIgnorePattern": "^_",
                "varsIgnorePattern": "^_",
            }
        ],
        "@typescript-eslint/await-thenable": "error",
        "@typescript-eslint/no-floating-promises": "error",
        "@typescript-eslint/no-misused-promises": [
            "error",
            {
                "checksVoidReturn": false,
            }
        ],
        "require-await": "off",
        "@typescript-eslint/require-await": "warn",
        "@typescript-eslint/switch-exhaustiveness-check": "error",
    },
};
