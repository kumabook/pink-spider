import {
  call,
  put,
  takeEvery,
} from 'redux-saga/effects';
import api from '../api/entry';
import {
  showProgress,
  hideProgress,
  showMessage,
} from '../actions/app';
import {
  index,
  show,
  update,
  creators,
} from '../actions/entry';
import { defaultPerPage } from '../config';

export function* fetchEntries({ payload: { page = 0, perPage = defaultPerPage, query, feedId } }) {
  try {
    yield put(showProgress());
    const items = yield call(api.index, page, perPage, query, feedId);
    yield put(creators.index.succeeded(items));
  } catch (e) {
    yield put(creators.index.failed(e));
    yield put(showMessage(e.message));
  } finally {
    yield put(hideProgress());
  }
}

export function* watchFetchEntries() {
  yield takeEvery(index.start, fetchEntries);
}

export function* fetchEntry({ payload }) {
  try {
    yield put(showProgress());
    const item = yield call(api.show, payload);
    yield put(creators.show.succeeded(item));
  } catch (e) {
    yield put(creators.show.failed(e));
    yield put(showMessage(e.message));
  } finally {
    yield put(hideProgress());
  }
}

export function* watchFetchEntry() {
  yield takeEvery(show.start, fetchEntry);
}

export function* updateEntry({ payload }) {
  try {
    yield put(showProgress());
    const item = yield call(api.update, payload);
    yield put(creators.update.succeeded(item));
    yield put(creators.index.start({ page: 0, perPage: defaultPerPage }));
  } catch (e) {
    yield put(creators.update.failed(e));
    yield put(showMessage(e.message));
  } finally {
    yield put(hideProgress());
  }
}

export function* watchUpdateEntry() {
  yield takeEvery(update.start, updateEntry);
}
