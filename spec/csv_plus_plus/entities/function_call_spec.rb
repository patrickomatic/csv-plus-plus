# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::FunctionCall do
  let(:entity) { described_class.new(:minus, [build(:number_one), build(:variable_foo)]) }

  subject { entity }

  describe '#initialize' do
    it 'sets @type' do
      expect(subject.type).to(eq(::CSVPlusPlus::Entities::Type::FunctionCall))
    end

    it 'sets @id' do
      expect(subject.id).to(eq(:minus))
    end
  end

  describe '#==' do
    it { is_expected.to(eq(build(:fn_call, name: :minus, arguments: [build(:number_one), build(:variable_foo)]))) }

    it { is_expected.not_to(eq(build(:fn_foo))) }
    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end

  describe '#evaluate' do
    let(:runtime) { build(:runtime) }

    subject { entity.evaluate(runtime) }

    it { is_expected.to(eq('MINUS(1, $$foo)')) }

    context 'with no args' do
      let(:entity) do
        build(
          :fn_call,
          name: :foo,
          arguments: [
            build(:cell_reference, ref: 'A'),
            build(:cell_reference, ref: 'B'),
            build(:cell_reference, ref: 'C')
          ]
        )
      end

      it { is_expected.to(eq('FOO(A, B, C)')) }
    end

    context 'with an infix function' do
      let(:entity) do
        build(
          :fn_call,
          name: :*,
          arguments: [
            build(:cell_reference, ref: 'A'),
            build(:cell_reference, ref: 'B')
          ],
          infix: true
        )
      end

      it { is_expected.to(eq('(A * B)')) }
    end
  end
end
