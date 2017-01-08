const gulp     = require('gulp');
const eslint   = require('gulp-eslint');

gulp.task('lint', () =>
          gulp.src(['web/js/**/*.{js,jsx}', 'web/test/**/*.{js,jsx}'])
              .pipe(eslint({ fix: true}))
              .pipe(eslint.format())
              .pipe(eslint.failAfterError())
         );

gulp.task('default', ['lint'], function () {});
