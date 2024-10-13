"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.appDir = void 0;
exports.appDir = (require('app-root-path')).toString();
const createError = require("http-errors");
const express = require("express");
require("../bot/bot.js");
const path = require("path");
const cookieParser = require("cookie-parser");
const logger = require("morgan");
const sassMiddleware = require("node-sass-middleware");
const router = require("./router");
const session = require("express-session");
const FileStore = require("session-file-store")(session);
const app = express();
require("./backup");
// view engine setup
app.set('views', path.join(exports.appDir, 'server/views'));
app.set('view engine', 'pug');
app.use(logger('dev'));
app.use(express.json());
app.use(express.urlencoded({ extended: false, limit: "5mb" }));
app.use(cookieParser());
const staticDirectory = path.join(exports.appDir, 'public');
app.use(session({
    cookie: { maxAge: 1000 * 60 * 60 * 24 },
    store: new FileStore(),
    secret: process.env.SECRET_KEY,
    resave: true,
    saveUninitialized: true,
}));
app.use(sassMiddleware({
    src: staticDirectory,
    dest: staticDirectory,
    indentedSyntax: true,
    sourceMap: true
}));
app.use(express.static(staticDirectory));
app.use(router);
// catch 404 and forward to error handler
app.use(function (req, res, next) {
    next(createError(404));
});
// error handler
app.use(function (err, req, res, next) {
    // set locals, only providing error in development
    res.locals.message = err.message;
    res.locals.error = req.app.get('env') === 'development' ? err : {};
    // render the error page
    res.status(err.status || 500);
    res.render('error');
});
process.on('unhandledRejection', error => {
    console.log('unhandledRejection', error);
});
module.exports = app;
//# sourceMappingURL=app.js.map