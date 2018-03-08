import {
  call,
  put,
  takeEvery,
} from 'redux-saga/effects';
import api from '../api/track';
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
} from '../actions/track';
import { defaultPerPage } from '../config';

export function* fetchTracks({ payload: { page = 0, perPage = defaultPerPage, query, entryId } }) {
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

export function* watchFetchTracks() {
  yield takeEvery(index.start, fetchTracks);
}

export function* fetchTrack({ payload }) {
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

export function* watchFetchTrack() {
  yield takeEvery(show.start, fetchTrack);
}

export function* updateTrack({ payload }) {
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

export function* watchUpdateTrack() {
  yield takeEvery(update.start, updateTrack);
}
