import {
  call,
  put,
  takeEvery,
} from 'redux-saga/effects';
import api from '../api/playlist';
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
} from '../actions/playlist';
import { defaultPerPage } from '../config';

export function* fetchPlaylists({
  payload: { page = 0, perPage = defaultPerPage, query, entryId },
}) {
  try {
    yield put(showProgress());
    const items = yield call(api.index, page, perPage, query, entryId);
    yield put(creators.index.succeeded(items));
  } catch (e) {
    yield put(creators.index.failed(e));
    yield put(showMessage(e.message));
  } finally {
    yield put(hideProgress());
  }
}

export function* watchFetchPlaylists() {
  yield takeEvery(index.start, fetchPlaylists);
}

export function* fetchPlaylist({ payload }) {
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

export function* watchFetchPlaylist() {
  yield takeEvery(show.start, fetchPlaylist);
}

export function* updatePlaylist({ payload }) {
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

export function* watchUpdatePlaylist() {
  yield takeEvery(update.start, updatePlaylist);
}
