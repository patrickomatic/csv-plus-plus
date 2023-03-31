# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Modifier::DataValidation do
  describe '#valid?' do
    let(:value) { nil }

    subject { described_class.new(value) }

    describe 'blank' do
      let(:value) { 'blank' }

      it { is_expected.to(be_valid) }

      context 'with an argument' do
        let(:value) { 'blank: foo' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'boolean' do
      let(:value) { 'boolean' }

      context ': false' do
        let(:value) { 'boolean: false' }
        it { is_expected.to(be_valid) }
      end

      context ': true' do
        let(:value) { 'boolean: true' }
        it { is_expected.to(be_valid) }
      end

      context ': true false' do
        let(:value) { 'boolean: true false' }
        it { is_expected.to(be_valid) }
      end

      context ': anything' do
        let(:value) { 'boolean: anything' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'custom_formula' do
      let(:value) { "custom_formula:'=A > 2'" }

      it { is_expected.to(be_valid) }
    end

    describe 'date_after' do
      let(:value) { "date_after: '11/1/2022'" }

      it { is_expected.to(be_valid) }

      context ': today' do
        let(:value) { 'date_after: today' }
        it { is_expected.to(be_valid) }
      end

      context ': invalid' do
        let(:value) { 'date_after: invalid' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'date_before' do
      let(:value) { 'date_before: 11/1/2022' }

      it { is_expected.to(be_valid) }

      context ': today' do
        let(:value) { 'date_before: today' }
        it { is_expected.to(be_valid) }
      end

      context ': invalid' do
        let(:value) { 'date_before: invalid' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'date_between' do
      let(:value) { "date_between:'11/1/2022 12/1/2022'" }

      it { is_expected.to(be_valid) }

      context ': today' do
        let(:value) { 'date_between: today' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'date_eq' do
      let(:value) { 'date_eq: 11/1/2022' }

      it { is_expected.to(be_valid) }

      context ': today' do
        let(:value) { 'date_eq: today' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'date_is_valid' do
      let(:value) { 'date_is_valid' }

      it { is_expected.to(be_valid) }

      context 'with an argument' do
        let(:value) { 'date_is_valid: foo' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'date_not_between' do
      let(:value) { 'date_not_between: 11/1/2022 12/1/2022' }

      it { is_expected.to(be_valid) }

      context ': today' do
        let(:value) { 'date_not_between: today' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'date_not_eq' do
      let(:value) { 'date_not_eq: 11/1/2022' }

      it { is_expected.to(be_valid) }

      context ': today' do
        let(:value) { 'date_not_eq: today' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'date_on_or_after' do
      let(:value) { 'date_on_or_after: 11/1/2022' }

      it { is_expected.to(be_valid) }

      context ': tomorrow' do
        let(:value) { 'date_on_or_after: tomorrow' }
        it { is_expected.to(be_valid) }
      end

      context ': invalid' do
        let(:value) { 'date_on_or_after: invalid' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'date_on_or_before' do
      let(:value) { 'date_on_or_before: 11/1/2022' }

      it { is_expected.to(be_valid) }

      context ': today' do
        let(:value) { 'date_on_or_before: today' }
        it { is_expected.to(be_valid) }
      end

      context ': invalid' do
        let(:value) { 'date_on_or_before: invalid' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'not_blank' do
      let(:value) { 'not_blank' }

      it { is_expected.to(be_valid) }

      context 'with an argument' do
        let(:value) { 'not_blank: foo' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'number_between' do
      let(:value) { 'number_between: 2 10' }

      it { is_expected.to(be_valid) }

      context ': foo' do
        let(:value) { 'number_between: foo' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'number_eq' do
      let(:value) { 'number_eq: 2' }

      it { is_expected.to(be_valid) }

      context ': foo' do
        let(:value) { 'number_eq: foo' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'number_greater' do
      let(:value) { 'number_greater: 2' }

      it { is_expected.to(be_valid) }

      context ': foo' do
        let(:value) { 'number_greater: foo' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'number_greater_than_eq' do
      let(:value) { 'number_greater_than_eq: 2' }

      it { is_expected.to(be_valid) }

      context ': foo' do
        let(:value) { 'number_greater_than_eq: foo' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'number_less' do
      let(:value) { 'number_less: 2' }

      it { is_expected.to(be_valid) }

      context ': foo' do
        let(:value) { 'number_less: foo' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'number_less_than_eq' do
      let(:value) { 'number_less_than_eq: 2' }

      it { is_expected.to(be_valid) }

      context ': foo' do
        let(:value) { 'number_less_than_eq: foo' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'number_not_between' do
      let(:value) { 'number_not_between: 2 10' }

      it { is_expected.to(be_valid) }

      context ': foo' do
        let(:value) { 'number_not_between: foo' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'number_not_eq' do
      let(:value) { 'number_not_eq: 2' }

      it { is_expected.to(be_valid) }

      context ': foo' do
        let(:value) { 'number_not_eq: foo' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'one_of_list' do
      let(:value) { 'one_of_list: 1 2 3' }

      it { is_expected.to(be_valid) }

      context ':' do
        let(:value) { 'one_of_list' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'one_of_range' do
      let(:value) { 'one_of_range: A1:B2' }

      it { is_expected.to(be_valid) }

      context ': foo' do
        let(:value) { 'one_of_range: too many args' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'text_contains' do
      let(:value) { 'text_ends_with: foo' }

      it { is_expected.to(be_valid) }

      context ':' do
        let(:value) { 'text_ends_with' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'text_ends_with' do
      let(:value) { 'text_ends_with: foo' }

      it { is_expected.to(be_valid) }

      context ':' do
        let(:value) { 'text_ends_with' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'text_eq' do
      let(:value) { 'text_eq: foo' }

      it { is_expected.to(be_valid) }

      context ':' do
        let(:value) { 'text_eq' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'text_is_email' do
      let(:value) { 'text_is_email' }

      it { is_expected.to(be_valid) }

      context 'with an argument' do
        let(:value) { 'text_is_email: foo' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'text_is_url' do
      let(:value) { 'text_is_url' }

      it { is_expected.to(be_valid) }

      context 'with an argument' do
        let(:value) { 'text_is_url: foo' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'text_not_contains' do
      let(:value) { 'text_not_contains: foo' }

      it { is_expected.to(be_valid) }

      context ':' do
        let(:value) { 'text_not_contains' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'text_not_eq' do
      let(:value) { 'text_not_eq: foo' }

      it { is_expected.to(be_valid) }

      context ':' do
        let(:value) { 'text_not_eq' }
        it { is_expected.not_to(be_valid) }
      end
    end

    describe 'text_starts_with' do
      let(:value) { 'text_starts_with: foo' }

      it { is_expected.to(be_valid) }

      context ':' do
        let(:value) { 'text_starts_with' }
        it { is_expected.not_to(be_valid) }
      end
    end
  end
end
