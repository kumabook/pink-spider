const gulp = require('gulp');
const eslint = require('gulp-eslint');

gulp.task('lint', () => {
  return gulp.src(['web/js/**/*.js', 'web/js/**/*.jsx'])
    .pipe(eslint({ fix: true}))
    .pipe(eslint.format())
    .pipe(eslint.failAfterError());
});

gulp.task('default', ['lint'], function () {});
