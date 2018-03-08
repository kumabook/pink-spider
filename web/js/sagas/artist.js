import {
  call,
  put,
  takeEvery,
} from 'redux-saga/effects';
import api from '../api/artist';
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
} from '../actions/artist';
import { defaultPerPage } from '../config';


export function* fetchArtists({ payload: { page = 0, perPage = defaultPerPage, query } }) {
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

export function* watchFetchArtists() {
  yield takeEvery(index.start, fetchArtists);
}

export function* fetchArtist({ payload }) {
  try {
    const items = yield call(api.show, payload);
    yield put(creators.show.succeeded(items));
  } catch (e) {
    yield put(creators.show.failed(e));
    yield put(showMessage(e.message));
  }
}

export function* watchFetchArtist() {
  yield takeEvery(show.start, fetchArtist);
}

export function* updateArtist({ payload }) {
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

export function* watchUpdateArtist() {
  yield takeEvery(update.start, updateArtist);
}
