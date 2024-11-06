import {FileStore} from "session-file-store";

export const appDir: string = (require('app-root-path')).toString();

import createError = require('http-errors');
import express = require("express");
import "../bot/bot.js";
import path = require('path');
import cookieParser = require('cookie-parser');
import logger = require('morgan');
import sassMiddleware = require('node-sass-middleware');
const router = require("./router");
import {Express} from "express";
import session = require("express-session");
const FileStore: FileStore = require("session-file-store")(session)
const app: Express = express();
import "./backup";

// view engine setup
app.set('views', path.join(appDir, 'server/views'));
app.set('view engine', 'pug');

app.use(logger('dev'));
app.use(express.json());
app.use(express.urlencoded({ extended: false, limit: "5mb" }));
app.use(cookieParser());

const staticDirectory = path.join(appDir, 'public');

app.use(session({
    cookie: { maxAge: 1000 * 60 * 60 * 24 },
    store: new FileStore(),
    secret: process.env.SECRET_KEY,
    resave: true,
    saveUninitialized:true,
}));

app.use(sassMiddleware({
    src: staticDirectory,
    dest: staticDirectory,
    indentedSyntax: true, // true = .sass and false = .scss
    sourceMap: true
}));

app.use(express.static(staticDirectory));

app.use(router);

// catch 404 and forward to error handler
app.use(function(req, res, next) {
    next(createError(404));
});

// error handler
app.use(function(err, req, res, next) {
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
