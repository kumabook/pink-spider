import React         from 'react';
import PropTypes     from 'prop-types';
import ReactPaginate from 'react-paginate';

const breakLabel = <a href="">...</a>;
const Paginate = ({ page, pageCount, onChange }) => (
  <ReactPaginate
    initialPage={page}
    previousLabel={'previous'}
    nextLabel={'next'}
    breakLabel={breakLabel}
    breakClassName={'break-me'}
    pageCount={pageCount}
    marginPagesDisplayed={2}
    pageRangeDisplayed={5}
    containerClassName={'pagination'}
    subContainerClassName={'pages pagination'}
    activeClassName={'active'}
    onPageChange={onChange}
  />
);

Paginate.propTypes = {
  page:      PropTypes.number.isRequired,
  pageCount: PropTypes.number.isRequired,
  onChange:  PropTypes.func.isRequired,
};

export default Paginate;
