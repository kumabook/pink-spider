import {
  call,
  put,
  takeEvery,
} from 'redux-saga/effects';
import api from '../api/feed';
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
} from '../actions/feed';
import { defaultPerPage } from '../config';

export function* fetchFeeds({ payload: { page = 0, perPage = defaultPerPage, query } }) {
  try {
    yield put(showProgress());
    const items = yield call(api.index, page, perPage, query);
    yield put(creators.index.succeeded(items));
  } catch (e) {
    yield put(creators.index.failed(e));
    yield put(showMessage(e.message));
  } finally {
    yield put(hideProgress());
  }
}

export function* watchFetchFeeds() {
  yield takeEvery(index.start, fetchFeeds);
}

export function* fetchFeed() {
  try {
    yield put(showProgress());
    const items = yield call(api.show);
    yield put(creators.show.succeeded(items));
  } catch (e) {
    yield put(creators.show.failed(e));
    yield put(showMessage(e.message));
  } finally {
    yield put(hideProgress());
  }
}

export function* watchFetchFeed() {
  yield takeEvery(show.start, fetchFeed);
}

export function* updateFeed({ payload }) {
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

export function* watchUpdateFeed() {
  yield takeEvery(update.start, updateFeed);
}
